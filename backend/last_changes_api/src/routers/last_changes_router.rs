use utils::ApiDocAddon;
use rocket::response::stream::{Event, EventStream};
use rocket::{Shutdown, State};
use rocket::tokio::sync::broadcast::{error::RecvError, Sender};
use rocket::tokio::select;
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use utils::AuthenticatedUser;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InventoryChangeEvent {
    /// UUID of the changed entity. For dependent tables the UUID of the parent, e.g. the inventory for `inventory_item`, the spell list for `spell_list_entry`, the owner for `spell_slots`, `concentration` and `spell_attack_modifier`.
    pub uuid: String,
    /// Name of the changed database table, e.g. `inventory`, `item_preset`, `spell_preset`, `spell_list`, `spell_slots`, `concentration`
    pub source: String,
    /// `INSERT`, `UPDATE` or `DELETE`
    #[serde(rename = "type")]
    pub change_type: String,
    /// Unix timestamp in milliseconds
    pub timestamp: u128,
}

/// SSE endpoint that streams inventory change events to authenticated clients
#[utoipa::path(
    get,
    path = "/lastChanges/stream",
    summary = "Subscribe to data change events",
    description = "Opens a Server-Sent Events (SSE) stream that pushes a notification for every change of an InventarWerk or ZauberWerk table (inventories, items, item presets, spell presets, spell lists, spell slots, concentrations, spell attack modifiers). Events are triggered by Postgres triggers and contain only what changed, not the new data.",
    responses(
        (status = 200, description = "Event stream established", content_type = "text/event-stream")
    ),
    security(("cookie_auth" = [])),
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
    modifiers(&ApiDocAddon),
    paths(
        last_changes_stream
    ),
    components(
        schemas(InventoryChangeEvent)
    ),
    tags(
        (name = "Last Changes", description = "Server-Sent Events for data changes via Postgres NOTIFY")
    )
)]
pub struct LastChangesApiDoc;
