use std::env;
use std::path::PathBuf;
use std::process::Stdio;

use repos::{server_major_version, DbPool};
use rocket::response::stream::ByteStream;
use rocket::tokio::{self, io::AsyncReadExt, process::Command};
use rocket::State;
use rocket_errors::anyhow::Result;
use utoipa::OpenApi;

use utils::{create_error, DmUser};

/// Directory where the Debian/PGDG packages `postgresql-client-<major>` install their binaries.
const PG_LIB_DIR: &str = "/usr/lib/postgresql";

#[derive(OpenApi)]
#[openapi(
    paths(create_database_backup),
    tags((name = "Backup", description = "Database backup endpoints"))
)]
pub struct BackupApiDoc;

/// Returns all installed `pg_dump` binaries as `(major version, path)`, sorted by version.
fn installed_pg_dumps() -> Vec<(u32, PathBuf)> {
    let Ok(entries) = std::fs::read_dir(PG_LIB_DIR) else {
        return Vec::new();
    };
    let mut pg_dumps: Vec<(u32, PathBuf)> = entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let major = entry.file_name().to_str()?.parse::<u32>().ok()?;
            let path = entry.path().join("bin").join("pg_dump");
            path.is_file().then_some((major, path))
        })
        .collect();
    pg_dumps.sort_by_key(|(major, _)| *major);
    pg_dumps
}

/// Selects the `pg_dump` matching the server version.
///
/// `pg_dump` can only dump servers of the same or an older major version, and a dump made by a
/// newer `pg_dump` may not restore into an older server. Therefore the oldest installed
/// `pg_dump` that is not older than the server is used, ideally exactly the server's version.
///
/// If no versioned `pg_dump` is installed (e.g. during local development on another
/// distribution), `pg_dump` from `PATH` is used.
fn select_pg_dump(server_major: u32) -> Result<PathBuf> {
    let installed = installed_pg_dumps();
    if installed.is_empty() {
        return Ok(PathBuf::from("pg_dump"));
    }
    installed
        .iter()
        .find(|(major, _)| *major >= server_major)
        .map(|(_, path)| path.clone())
        .ok_or_else(|| {
            let versions: Vec<String> = installed.iter().map(|(m, _)| m.to_string()).collect();
            create_error(&format!(
                "No pg_dump for PostgreSQL {server_major} installed (installed: {}). \
                 Rebuild the image with this version in PG_VERSIONS.",
                versions.join(", ")
            ))
        })
}

#[utoipa::path(
    get,
    path = "/backup/database",
    summary = "Create a database backup",
    description = "Creates a complete PostgreSQL database backup in SQL format using the pg_dump matching the server version. Streams the backup directly for memory efficiency. If pg_dump fails while streaming, the file ends with an '-- ERROR' line. Only accessible for DMs.",
    responses(
        (status = 200, description = "Database backup as SQL dump", content_type = "application/sql"),
        (status = 403, description = "Forbidden - User is not a DM"),
        (status = 500, description = "Internal server error, e.g. no matching pg_dump or pg_dump failed")
    ),
    security(("bearer_auth" = [])),
    tag = "Backup"
)]
#[get("/backup/database")]
pub async fn create_database_backup(
    _dm: DmUser,
    pool: &State<DbPool>,
) -> Result<ByteStream![Vec<u8>]> {
    let database_url =
        env::var("DATABASE_URL").map_err(|_| create_error("DATABASE_URL not set"))?;

    let server_major = server_major_version(pool).await?;
    let pg_dump = select_pg_dump(server_major)?;

    // pg_dump understands the connection URI directly (incl. URL-encoded passwords and
    // parameters like ?sslmode=...), so it does not need to be parsed here.
    let mut child = Command::new(&pg_dump)
        .args([
            "--format=plain",
            "--clean",
            "--if-exists",
            "--no-owner",
            "--no-privileges",
        ])
        .arg(format!("--dbname={database_url}"))
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| {
            create_error(&format!(
                "Failed to start {}: {e}. Make sure the PostgreSQL client tools are installed.",
                pg_dump.display()
            ))
        })?;

    let mut stdout = child
        .stdout
        .take()
        .ok_or_else(|| create_error("Failed to capture pg_dump stdout"))?;
    let mut stderr = child
        .stderr
        .take()
        .ok_or_else(|| create_error("Failed to capture pg_dump stderr"))?;

    // Drain stderr concurrently, so pg_dump never blocks on a full pipe
    let stderr_task = tokio::spawn(async move {
        let mut output = String::new();
        let _ = stderr.read_to_string(&mut output).await;
        output
    });

    // Wait for the first chunk before responding. If pg_dump fails right away (wrong
    // credentials, version mismatch, ...), it writes nothing to stdout; answer with an
    // error then instead of a 200 response containing an empty backup.
    let mut buffer = vec![0u8; 8192];
    let first_len = stdout
        .read(&mut buffer)
        .await
        .map_err(|e| create_error(&format!("Failed to read pg_dump output: {e}")))?;
    if first_len == 0 {
        let status = child.wait().await;
        let error_output = stderr_task.await.unwrap_or_default();
        return Err(create_error(&format!(
            "pg_dump failed ({}): {}",
            status.map_or_else(|e| e.to_string(), |s| s.to_string()),
            error_output.trim()
        )));
    }
    let first_chunk = buffer[..first_len].to_vec();

    let header = format!(
        "-- InventarWerk Database Backup\n\
         -- Generated at: {}\n\
         -- PostgreSQL server version: {server_major}\n\
         -- pg_dump: {}\n\n",
        chrono::Utc::now(),
        pg_dump.display()
    );

    Ok(ByteStream! {
        yield header.into_bytes();
        yield first_chunk;

        loop {
            match stdout.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => yield buffer[..n].to_vec(),
                Err(e) => {
                    eprintln!("Error reading pg_dump output: {e}");
                    break;
                }
            }
        }

        // The response status is already sent at this point. Mark a failed dump in the
        // file itself, so an incomplete backup is not mistaken for a complete one.
        match child.wait().await {
            Ok(status) if status.success() => {}
            status => {
                let error_output = stderr_task.await.unwrap_or_default();
                let status = status.map_or_else(|e| e.to_string(), |s| s.to_string());
                eprintln!("pg_dump failed ({status}): {}", error_output.trim());
                yield format!(
                    "\n-- ERROR: THIS BACKUP IS INCOMPLETE. pg_dump failed ({status}): {}\n",
                    error_output.trim().replace('\n', "\n-- ")
                )
                .into_bytes();
            }
        }
    })
}
