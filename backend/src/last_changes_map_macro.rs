use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref GLOBAL_MAP: Mutex<HashMap<String, u128>> = Mutex::new(HashMap::new());
}

#[macro_export]
macro_rules! report_change_on_inventory {
    ($id:expr) => {
        use std::time::{SystemTime, UNIX_EPOCH};
        use crate::last_changes_map_macro::GLOBAL_MAP;
        let start = SystemTime::now();
        let duration = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let timestamp_in_seconds = duration.as_millis();
        let mut map = GLOBAL_MAP.lock().unwrap();
        map.insert($id.to_string(), (timestamp_in_seconds));
    };
}

#[macro_export]
macro_rules! get_last_inventory_change {
    ($id:expr) => {
        *crate::last_changes_map_macro::GLOBAL_MAP.lock().unwrap().get($id).unwrap_or(&0)
    };
}
