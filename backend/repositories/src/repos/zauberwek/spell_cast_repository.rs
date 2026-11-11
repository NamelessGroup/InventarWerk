use sqlx::{Error, PgPool};

pub struct SpellCastRepository {
    pool: PgPool,
}

/// Result of a spell cast attempt.
pub enum CastOutcome {
    /// The spell was cast.
    Cast {
        /// Available spell slots of the cast level after casting. `None` for cantrips (no slot consumed).
        remaining_slots: Option<i32>,
        /// The spell the caster was concentrating on before, if the cast replaced a concentration.
        previous_concentration: Option<String>,
    },
    /// No spell slot of the requested level was available. Nothing was changed.
    NoSlotAvailable,
}

impl SpellCastRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Casts a spell in a single transaction:
    /// - consumes one spell slot of `cast_level` (skipped for `cast_level == 0`)
    /// - sets the caster's concentration to the spell if `concentration` is true
    ///
    /// If no slot is available, nothing is changed and `CastOutcome::NoSlotAvailable` is returned.
    pub async fn cast_spell(
        &self,
        owner_uuid: &str,
        spell_preset_uuid: &str,
        cast_level: i32,
        concentration: bool,
    ) -> Result<CastOutcome, Error> {
        let mut tx = self.pool.begin().await?;

        let remaining_slots = if cast_level > 0 {
            // Atomic decrement: concurrent casts can never take the same last slot.
            let remaining = sqlx::query_scalar!(
                "UPDATE spell_slots
                 SET spell_slot_available = spell_slot_available - 1
                 WHERE owner_uuid = $1 AND spell_slot_level = $2 AND spell_slot_available > 0
                 RETURNING spell_slot_available",
                owner_uuid,
                cast_level
            )
            .fetch_optional(&mut *tx)
            .await?;

            match remaining {
                Some(remaining) => Some(remaining),
                // Dropping the transaction rolls it back.
                None => return Ok(CastOutcome::NoSlotAvailable),
            }
        } else {
            None
        };

        let previous_concentration = if concentration {
            let previous = sqlx::query_scalar!(
                "SELECT spell_preset_uuid FROM concentration WHERE owner_uuid = $1 FOR UPDATE",
                owner_uuid
            )
            .fetch_optional(&mut *tx)
            .await?;

            sqlx::query!(
                "INSERT INTO concentration (owner_uuid, spell_preset_uuid)
                 VALUES ($1, $2)
                 ON CONFLICT (owner_uuid)
                 DO UPDATE SET spell_preset_uuid = $2",
                owner_uuid,
                spell_preset_uuid
            )
            .execute(&mut *tx)
            .await?;

            previous
        } else {
            None
        };

        tx.commit().await?;

        Ok(CastOutcome::Cast {
            remaining_slots,
            previous_concentration,
        })
    }
}
