pub struct LockController {
    locked: bool
}

impl LockController {
    pub fn new (init:bool)  -> Self {
        return Self {locked: init};
    }
    pub fn is_locked (&self) -> bool {
        return self.locked
    }

    pub fn toggle_lock(&mut self) {
        self.locked = !(self.locked);
    }
}