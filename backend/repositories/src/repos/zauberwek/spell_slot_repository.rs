use crate::model_zauberwerk::SpellSlot;
use sqlx::{Error, PgPool};

pub struct SpellSlotRepository {
    pool: PgPool,
}

/// Result of setting the available spell slots of a level.
pub enum AvailableSlotsUpdate {
    /// The available spell slots were set.
    Updated,
    /// No maximum was set for the level yet. Nothing was changed.
    LevelNotFound,
    /// The new amount exceeds the maximum of the level. Nothing was changed.
    ExceedsMaximum,
}

impl SpellSlotRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Updates the maximum spell slots for a specific level and owner.
    ///
    /// A new level is created with all slots available. If the maximum of an existing level is
    /// lowered below the available slots, the available slots are lowered to the new maximum.
    pub async fn modify_max_spell_slots(&self, owner_uuid: &str, spell_slot_level: i32, new_maxima: i32) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO spell_slots (owner_uuid, spell_slot_level, spell_slot_maxima, spell_slot_available)
             VALUES ($1, $2, $3, $3)
             ON CONFLICT (owner_uuid, spell_slot_level)
             DO UPDATE SET spell_slot_maxima = $3,
                           spell_slot_available = LEAST(spell_slots.spell_slot_available, $3)",
            owner_uuid,
            spell_slot_level,
            new_maxima
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Updates the available spell slots for a specific level and owner.
    ///
    /// Nothing is changed if the level does not exist or `new_available` exceeds its maximum.
    pub async fn modify_current_spell_slots(&self, owner_uuid: &str, spell_slot_level: i32, new_available: i32) -> Result<AvailableSlotsUpdate, Error> {
        let mut tx = self.pool.begin().await?;

        // Lock the row, so the maximum can not change between check and update.
        let maxima = sqlx::query_scalar!(
            "SELECT spell_slot_maxima FROM spell_slots
             WHERE owner_uuid = $1 AND spell_slot_level = $2
             FOR UPDATE",
            owner_uuid,
            spell_slot_level
        )
        .fetch_optional(&mut *tx)
        .await?;

        match maxima {
            None => return Ok(AvailableSlotsUpdate::LevelNotFound),
            Some(maxima) if new_available > maxima => return Ok(AvailableSlotsUpdate::ExceedsMaximum),
            Some(_) => {}
        }

        sqlx::query!(
            "UPDATE spell_slots
             SET spell_slot_available = $3
             WHERE owner_uuid = $1 AND spell_slot_level = $2",
            owner_uuid,
            spell_slot_level,
            new_available
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(AvailableSlotsUpdate::Updated)
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