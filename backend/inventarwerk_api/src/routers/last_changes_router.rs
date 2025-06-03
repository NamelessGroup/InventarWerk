use std::collections::HashMap;

use repos::repos::inventory_repository::InventoryRepository;
use rocket::{serde::json::Json, State};

use utoipa::OpenApi;

use crate::get_last_inventory_change;
use rocket_errors::anyhow::Result;

#[utoipa::path(
    get,
    path = "/lastChanges",
    summary = "Retrieve last change timestamps",
    description = "Returns the last change timestamps for all inventories accessible to the authenticated user.",
    responses(
        (status = 200, description = "Map of last changes keyed by inventory UUID")
    ),
    security(("bearer_auth" = [])),
    tag = "Last Changes"
)]
#[get("/lastChanges")]
pub async fn last_changes(
    user: super::AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
) -> Result<Json<HashMap<String, u128>>> {
    let mut inv_hash: HashMap<String, u128> = HashMap::new();
    let invs = inv_rep
        .get_owned_and_readable_inventory_ids(&user.user_id)
        .await?;
    for i in invs {
        inv_hash.insert(i.clone(), get_last_inventory_change!(&i));
    }
    Ok(Json(inv_hash))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        last_changes
    ),
    tags(
        (name = "Last Changes", description = "Endpoint to get last changes")
    )
)]
pub struct LastChangesApiDoc;
