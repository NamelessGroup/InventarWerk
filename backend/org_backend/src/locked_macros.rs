use std::sync::Mutex;

use lazy_static::lazy_static;

//! # Locked Macros
//!
//! This module provides global state and macros for managing a global "locked" status,
//! which can be toggled or queried at runtime. The lock is stored as a thread-safe global boolean.
//!
//! ## Macros
//! - [`lock_toggle!()`]: Toggles the global locked status.
//! - [`locked_status!()`]: Returns the current locked status as a boolean.


/// Global, thread-safe boolean indicating the locked status.
lazy_static! {
    pub static ref GLOBAL_LOCKED: Mutex<bool> = Mutex::new(true);
}

/// Macro to toggle the global locked status.
#[macro_export]
macro_rules! lock_toggle {
    () => {
        use crate::locked_macros::GLOBAL_LOCKED;
        let mut locked = GLOBAL_LOCKED.lock().unwrap();
        *locked = !*locked;
    };
}

/// Macro to get the current global locked status.
#[macro_export]
macro_rules! locked_status {
    () => {
        *crate::locked_macros::GLOBAL_LOCKED.lock().unwrap()
    };
}
