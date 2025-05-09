use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref GLOBAL_MAP: Mutex<HashMap<String, u128>> = Mutex::new(HashMap::new());
}

/// Macro to report a change on an inventory by updating its last change timestamp to the current time (in milliseconds since UNIX epoch).
#[macro_export]
macro_rules! report_change_on_inventory {
    ($id:expr) => {
        use crate::last_changes_map_macro::GLOBAL_MAP;
        use std::time::{SystemTime, UNIX_EPOCH};
        let start = SystemTime::now();
        let duration = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let timestamp_in_seconds = duration.as_millis();
        let mut map = GLOBAL_MAP.lock().unwrap();
        map.insert($id.to_string(), (timestamp_in_seconds));
    };
}

/// Macro to get the last change timestamp for an inventory by UUID.
///
/// Returns `0` if no timestamp is set for the given inventory.
#[macro_export]
macro_rules! get_last_inventory_change {
    ($id:expr) => {
        *crate::last_changes_map_macro::GLOBAL_MAP
            .lock()
            .unwrap()
            .get($id)
            .unwrap_or(&0)
    };
}
