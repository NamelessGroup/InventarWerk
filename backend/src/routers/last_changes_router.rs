use std::collections::HashMap;

use rocket::{serde::json::Json, State};

use crate::{controller::{inventory_controller::InventoryController, CStat}, get_last_inventory_change};


#[get("/lastChanges")]
pub async fn last_changes(user: super::AuthenticatedUser, inv_con: &State<InventoryController>) -> Result<Json<HashMap<String, u64>>, CStat> {
    let mut inv_hash: HashMap<String, u64> = HashMap::new();
    let invs = inv_con.get_all_inventories_ids(user.user_id)?;
    for i in invs {
        inv_hash.insert(i.clone(), get_last_inventory_change!(&i));
    }
    Ok(Json(inv_hash))
}