use crate::dbmod::DbPool;

#[derive(Clone)]
pub struct ItemPresetController {
    db: DbPool
}

impl ItemPresetController {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }
}