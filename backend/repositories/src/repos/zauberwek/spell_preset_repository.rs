use crate::model::{FrontendItem, FullFrontendInventory, InventoryItem, RawInventory};
use anyhow::{self, Result};
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct SpellPresetRepository {
    pool: PgPool,
}

impl SpellPresetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    
}