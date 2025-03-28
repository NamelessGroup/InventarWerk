use std::sync::Mutex;

pub struct LockController {
    locked: Mutex<bool>
}

impl LockController {
    pub fn new (init:bool)  -> Self {
        return Self {locked: init.into()};
    }
    pub fn is_locked (&self) -> bool {
        let lock_mut = self.locked.lock().expect("Mutex-Fehler");
        return *lock_mut
    }

    pub fn toggle_lock(&self) {
        let mut lock_mut = self.locked.lock().expect("Mutex-Fehler");
        *lock_mut = !(*lock_mut);
    }
}