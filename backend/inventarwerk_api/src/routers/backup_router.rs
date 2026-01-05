use rocket::State;
use rocket::response::stream::ByteStream;
use rocket_errors::anyhow::Result;
use utoipa::OpenApi;
use std::env;
use std::process::{Command, Stdio};
use tokio::io::{AsyncReadExt, BufReader};

use repos::repos::user_repository::UserRepository;
use utils::{create_error, user_is_dm, AuthenticatedUser};

#[derive(OpenApi)]
#[openapi(
    paths(create_database_backup),
    tags((name = "Backup", description = "Database backup endpoints"))
)]
pub struct BackupApiDoc;

#[utoipa::path(
    get,
    path = "/backup/database",
    summary = "Create a database backup",
    description = "Creates a complete PostgreSQL database backup in SQL format using pg_dump. Streams the backup directly for memory efficiency. Only accessible for DMs.",
    responses(
        (status = 200, description = "Database backup as SQL dump", content_type = "application/sql"),
        (status = 403, description = "Forbidden - User is not a DM"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = [])),
    tag = "Backup"
)]
#[get("/backup/database")]
pub async fn create_database_backup(
    user: AuthenticatedUser,
    usr_rep: &State<UserRepository>,
) -> Result<ByteStream![Vec<u8>]> {
    // Check if user is DM
    if !user_is_dm(usr_rep, user.user_id).await? {
        return Err(create_error("Forbidden: Only DMs can create backups"));
    }

    // Get database URL from environment
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| create_error("DATABASE_URL not set"))?;

    // Parse DATABASE_URL (format: postgres://user:password@host:port/dbname)
    let url_parts: Vec<&str> = database_url.split("://").collect();
    if url_parts.len() != 2 {
        return Err(create_error("Invalid DATABASE_URL format"));
    }

    let remaining = url_parts[1];
    let (credentials, host_db) = remaining.split_once('@')
        .ok_or_else(|| create_error("Invalid DATABASE_URL format"))?;
    
    let (user_pass, host_port_db) = (credentials, host_db);
    let (username, password) = user_pass.split_once(':')
        .unwrap_or((user_pass, ""));
    
    let (host_port, dbname) = host_port_db.split_once('/')
        .ok_or_else(|| create_error("Invalid DATABASE_URL format"))?;
    
    let (host, port) = host_port.split_once(':')
        .unwrap_or((host_port, "5432"));

    // Start pg_dump process
    let mut child = Command::new("pg_dump")
        .args([
            "--format=plain",
            "--clean",
            "--if-exists",
            "--no-owner",
            "--no-privileges",
            "-h", host,
            "-p", port,
            "-U", username,
            dbname,
        ])
        .env("PGPASSWORD", password)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| create_error(&format!("Failed to start pg_dump: {}. Make sure PostgreSQL client tools are installed.", e)))?;

    let stdout = child.stdout.take()
        .ok_or_else(|| create_error("Failed to capture pg_dump stdout"))?;

    Ok(ByteStream! {
        
        // Add header
        let header = format!(
            "-- InventarWerk Database Backup\n-- Generated at: {}\n-- \n\n",
            chrono::Utc::now()
        );
        yield header.into_bytes();

        // Stream the pg_dump output
        let mut reader = BufReader::new(tokio::process::ChildStdout::from_std(stdout).unwrap());
        let mut buffer = vec![0; 8192];

        loop {
            match reader.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => yield buffer[..n].to_vec(),
                Err(e) => {
                    eprintln!("Error reading pg_dump output: {}", e);
                    break;
                }
            }
        }
    })
}
