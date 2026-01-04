use crate::model_zauberwerk::SpellSlot;
use sqlx::{Error, PgPool};

pub struct SpellSlotRepository {
    pool: PgPool,
}

impl SpellSlotRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Updates the maximum spell slots for a specific level and owner.
    pub async fn modify_max_spell_slots(&self, owner_uuid: &str, spell_slot_level: i32, new_maxima: i32) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO spell_slots (owner_uuid, spell_slot_level, spell_slot_maxima, spell_slot_available)
             VALUES ($1, $2, $3, $3)
             ON CONFLICT (owner_uuid, spell_slot_level) 
             DO UPDATE SET spell_slot_maxima = $3",
            owner_uuid,
            spell_slot_level,
            new_maxima
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Updates the available spell slots for a specific level and owner.
    pub async fn modify_current_spell_slots(&self, owner_uuid: &str, spell_slot_level: i32, new_available: i32) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE spell_slots 
             SET spell_slot_available = $3
             WHERE owner_uuid = $1 AND spell_slot_level = $2",
            owner_uuid,
            spell_slot_level,
            new_available
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Retrieves the maximum spell slots for a specific level and owner.
    pub async fn get_max_spell_slots(&self, owner_uuid: &str, spell_slot_level: i32) -> Result<i32, Error> {
        let result = sqlx::query!(
            "SELECT spell_slot_maxima FROM spell_slots WHERE owner_uuid = $1 AND spell_slot_level = $2",
            owner_uuid,
            spell_slot_level
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(result.spell_slot_maxima)
    }

    /// Retrieves the available spell slots for a specific level and owner.
    pub async fn get_current_spell_slots(&self, owner_uuid: &str, spell_slot_level: i32) -> Result<i32, Error> {
        let result = sqlx::query!(
            "SELECT spell_slot_available FROM spell_slots WHERE owner_uuid = $1 AND spell_slot_level = $2",
            owner_uuid,
            spell_slot_level
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(result.spell_slot_available)
    }

    /// Retrieves all spell slots for a specific owner.
    pub async fn get_all_spell_slots(&self, owner_uuid: &str) -> Result<Vec<SpellSlot>, Error> {
        let slots = sqlx::query_as!(
            SpellSlot,
            "SELECT owner_uuid, spell_slot_level, spell_slot_maxima, spell_slot_available 
             FROM spell_slots 
             WHERE owner_uuid = $1
             ORDER BY spell_slot_level",
            owner_uuid
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(slots)
    }

    /// Resets available spell slots to maximum for all levels of an owner.
    pub async fn reset_spell_slots(&self, owner_uuid: &str) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE spell_slots 
             SET spell_slot_available = spell_slot_maxima 
             WHERE owner_uuid = $1",
            owner_uuid
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}