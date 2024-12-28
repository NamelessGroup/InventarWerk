use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref GLOBAL_MAP: Mutex<HashMap<String, (u64, String)>> = Mutex::new(HashMap::new());
}

#[macro_export]
macro_rules! add_to_global_map {
    ($id:expr, $operation:expr) => {
        use std::time::{SystemTime, UNIX_EPOCH};
        use crate::last_changes_map_macro::GLOBAL_MAP;
        let start = SystemTime::now();
        let duration = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let timestamp_in_seconds = duration.as_secs();
        let mut map = GLOBAL_MAP.lock().unwrap();
        map.insert($id.to_string(), (timestamp_in_seconds, $operation));
    };
}

#[macro_export]
macro_rules! get_from_global_map {
    ($key:expr) => {
        let map = GLOBAL_MAP.lock().unwrap();
        map.get($key)
    };
}
