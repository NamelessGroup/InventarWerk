use std::collections::HashMap;

use inv_rep::repos::inventory_repository::InventoryRepository;
use rocket::{serde::json::Json, State};

use crate::get_last_inventory_change;
use rocket_errors::anyhow::Result;

//! # Last Changes Router
//!
//! This module provides an endpoint to retrieve the last change timestamps for all inventories accessible to the authenticated user.
//!
//! ## Endpoints
//! - `GET /lastChanges`: Returns a map of inventory UUIDs to their last change timestamp (as `u128`).


/// Returns the last change timestamps for all inventories accessible to the authenticated user.
///
/// # Authentication
/// Requires authentication.
///
/// # Returns
/// A JSON map where the key is the inventory UUID and the value is the last change timestamp (`u128`).
#[get("/lastChanges")]
pub async fn last_changes(user: super::AuthenticatedUser, inv_rep: &State<InventoryRepository>) -> Result<Json<HashMap<String, u128>>> {
    let mut inv_hash: HashMap<String, u128> = HashMap::new();
    let invs = inv_rep.get_user_inventory_ids(&user.user_id).await?;
    for i in invs {
        inv_hash.insert(i.clone(), get_last_inventory_change!(&i));
    }
    Ok(Json(inv_hash))
}