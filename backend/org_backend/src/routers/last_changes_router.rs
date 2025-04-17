use std::collections::HashMap;

use inv_rep::repos::inventory_repository::InventoryRepository;
use rocket::{serde::json::Json, State};

use crate::get_last_inventory_change;
use rocket_errors::anyhow::Result;


#[get("/lastChanges")]
pub async fn last_changes(user: super::AuthenticatedUser, inv_rep: &State<InventoryRepository>) -> Result<Json<HashMap<String, u128>>> {
    let mut inv_hash: HashMap<String, u128> = HashMap::new();
    let invs = inv_rep.get_user_inventory_ids(&user.user_id).await?;
    for i in invs {
        inv_hash.insert(i.clone(), get_last_inventory_change!(&i));
    }
    Ok(Json(inv_hash))
}