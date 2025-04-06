use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref GLOBAL_LOCKED: Mutex<bool> = Mutex::new(true);
}

#[macro_export]
macro_rules! lock_toggle {
    () => {
        use crate::locked_macros::GLOBAL_LOCKED;
        let mut locked = GLOBAL_LOCKED.lock().unwrap();
        *locked = !*locked;
    };
}

#[macro_export]
macro_rules! locked_status {
    () => {
        *crate::locked_macros::GLOBAL_LOCKED.lock().unwrap()
    };
}
