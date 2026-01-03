use rocket::response::stream::{Event, EventStream};
use rocket::{Shutdown, State};
use rocket::tokio::sync::broadcast::{error::RecvError, Sender};
use rocket::tokio::select;
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use utils::AuthenticatedUser;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InventoryChangeEvent {
    pub inventory_uuid: String,
    pub timestamp: u128,
}

/// SSE endpoint that streams inventory change events to authenticated clients
#[utoipa::path(
    get,
    path = "/lastChanges/stream",
    summary = "Subscribe to inventory change events",
    description = "Opens a Server-Sent Events (SSE) stream that pushes real-time notifications when inventories are modified. Events are triggered by Postgres triggers.",
    responses(
        (status = 200, description = "Event stream established", content_type = "text/event-stream")
    ),
    security(("bearer_auth" = [])),
    tag = "Last Changes"
)]
#[get("/lastChanges/stream")]
pub async fn last_changes_stream(
    _user: AuthenticatedUser,
    queue: &State<Sender<InventoryChangeEvent>>,
    mut end: Shutdown,
) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        last_changes_stream
    ),
    components(
        schemas(InventoryChangeEvent)
    ),
    tags(
        (name = "Last Changes", description = "Server-Sent Events for inventory changes via Postgres NOTIFY")
    )
)]
pub struct LastChangesApiDoc;
