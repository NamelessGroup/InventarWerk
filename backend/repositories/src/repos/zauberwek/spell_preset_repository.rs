use crate::model_zauberwerk::SpellPreset;
use anyhow::{self, Result};
use sqlx::{Error, PgPool, Postgres, Transaction};
use uuid::Uuid;

pub struct SpellPresetRepository {
    pool: PgPool,
}

impl SpellPresetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Retrieves a spell preset by its UUID.
    pub async fn get_spell_preset(&self, spell_preset_uuid: &str) -> Result<SpellPreset, Error> {
        let result = sqlx::query!(
            r#"
            SELECT 
                sp.uuid, sp.name, sp.level, sp.description, sp.time, 
                sp.concentration, sp.duration, sp.range, sp.school, sp.condition_inflict, sp.saving_throw, sp.affects_creature_type, sp.damage_inflict, sp.spell_attack, sp.ritual, sp.damage_immune, sp.ability_check, sp.damage_resist, sp.condition_immune, sp.higher_level,
                COALESCE(array_agg(DISTINCT spc.component_name) FILTER (WHERE spc.component_name IS NOT NULL), '{}') as "components!",
                COALESCE(array_agg(DISTINCT spclass.spell_preset_class) FILTER (WHERE spclass.spell_preset_class IS NOT NULL), '{}') as "class!",
                COALESCE(array_agg(DISTINCT spsc.spell_preset_subclass) FILTER (WHERE spsc.spell_preset_subclass IS NOT NULL), '{}') as "sub_class!",
                COALESCE(array_agg(DISTINCT spsp.spell_preset_species) FILTER (WHERE spsp.spell_preset_species IS NOT NULL), '{}') as "species!",
                COALESCE(array_agg(DISTINCT spf.spell_preset_feats) FILTER (WHERE spf.spell_preset_feats IS NOT NULL), '{}') as "feats!"
            FROM spell_preset sp
            LEFT JOIN spell_preset_component spc ON sp.uuid = spc.spell_preset_uuid
            LEFT JOIN spell_preset_class spclass ON sp.uuid = spclass.spell_preset_uuid
            LEFT JOIN spell_preset_subclass spsc ON sp.uuid = spsc.spell_preset_uuid
            LEFT JOIN spell_preset_species spsp ON sp.uuid = spsp.spell_preset_uuid
            LEFT JOIN spell_preset_feats spf ON sp.uuid = spf.spell_preset_uuid
            WHERE sp.uuid = $1
            GROUP BY sp.uuid
            "#,
            spell_preset_uuid
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(SpellPreset {
            uuid: result.uuid,
            name: result.name,
            level: result.level,
            description: result.description,
            time: result.time,
            components: result.components,
            concentration: result.concentration,
            duration: result.duration,
            school: result.school,
            range: result.range,
            class: result.class,
            sub_class: result.sub_class,
            species: result.species,
            feats: result.feats,
            condition_inflict: result.condition_inflict,
            saving_throw: result.saving_throw,
            affects_creature_type: result.affects_creature_type,
            damage_inflict: result.damage_inflict,
            spell_attack: result.spell_attack,
            ritual: result.ritual,
            damage_immune: result.damage_immune,
            ability_check: result.ability_check,
            damage_resist: result.damage_resist,
            condition_immune: result.condition_immune,
            higher_level: result.higher_level,
        })
    }

    /// Creates a new spell preset. If `uuid` is empty, a new UUID is generated.
    /// Returns the UUID of the created preset.
    pub async fn create_spell_preset(&self, spell_preset: SpellPreset) -> Result<String, Error> {
        let id = if spell_preset.uuid.is_empty() {
            Uuid::new_v4().to_string()
        } else {
            spell_preset.uuid.clone()
        };

        let mut tx = self.pool.begin().await?;

        sqlx::query!(
            "INSERT INTO spell_preset (uuid, name, level, description, time, concentration, duration, range, school,
                 condition_inflict, saving_throw, affects_creature_type, damage_inflict, spell_attack, ritual, damage_immune, ability_check, damage_resist, condition_immune, higher_level)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)",
            id,
            spell_preset.name,
            spell_preset.level,
            spell_preset.description,
            spell_preset.time,
            spell_preset.concentration,
            spell_preset.duration,
            spell_preset.range,
            spell_preset.school,
            spell_preset.condition_inflict,
            spell_preset.saving_throw,
            spell_preset.affects_creature_type,
            spell_preset.damage_inflict,
            spell_preset.spell_attack,
            spell_preset.ritual,
            spell_preset.damage_immune,
            spell_preset.ability_check,
            spell_preset.damage_resist,
            spell_preset.condition_immune,
            spell_preset.higher_level
        )
        .execute(&mut *tx)
        .await?;

        Self::insert_details(&mut tx, &id, &spell_preset).await?;

        tx.commit().await?;
        Ok(id)
    }

    /// Modifies an existing spell preset in place.
    ///
    /// The preset keeps its UUID, so spell list entries and concentrations referencing it
    /// stay intact. Components, classes, subclasses, species and feats are replaced.
    ///
    /// Returns `false` if no preset with the given UUID exists; nothing is changed then.
    pub async fn modify_spell_preset(&self, spell_preset: SpellPreset) -> Result<bool, Error> {
        let mut tx = self.pool.begin().await?;

        let updated = sqlx::query!(
            "UPDATE spell_preset SET
                name = $2, level = $3, description = $4, time = $5, concentration = $6, duration = $7,
                range = $8, school = $9, condition_inflict = $10, saving_throw = $11, affects_creature_type = $12,
                damage_inflict = $13, spell_attack = $14, ritual = $15, damage_immune = $16, ability_check = $17,
                damage_resist = $18, condition_immune = $19, higher_level = $20
             WHERE uuid = $1",
            spell_preset.uuid,
            spell_preset.name,
            spell_preset.level,
            spell_preset.description,
            spell_preset.time,
            spell_preset.concentration,
            spell_preset.duration,
            spell_preset.range,
            spell_preset.school,
            spell_preset.condition_inflict,
            spell_preset.saving_throw,
            spell_preset.affects_creature_type,
            spell_preset.damage_inflict,
            spell_preset.spell_attack,
            spell_preset.ritual,
            spell_preset.damage_immune,
            spell_preset.ability_check,
            spell_preset.damage_resist,
            spell_preset.condition_immune,
            spell_preset.higher_level
        )
        .execute(&mut *tx)
        .await?
        .rows_affected();

        if updated == 0 {
            // Dropping the transaction rolls it back.
            return Ok(false);
        }

        let id = &spell_preset.uuid;
        sqlx::query!("DELETE FROM spell_preset_component WHERE spell_preset_uuid = $1", id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!("DELETE FROM spell_preset_class WHERE spell_preset_uuid = $1", id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!("DELETE FROM spell_preset_subclass WHERE spell_preset_uuid = $1", id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!("DELETE FROM spell_preset_species WHERE spell_preset_uuid = $1", id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!("DELETE FROM spell_preset_feats WHERE spell_preset_uuid = $1", id)
            .execute(&mut *tx)
            .await?;

        Self::insert_details(&mut tx, id, &spell_preset).await?;

        tx.commit().await?;
        Ok(true)
    }

    /// Inserts components, classes, subclasses, species and feats of a spell preset.
    async fn insert_details(
        tx: &mut Transaction<'_, Postgres>,
        id: &str,
        spell_preset: &SpellPreset,
    ) -> Result<(), Error> {
        for component in &spell_preset.components {
            sqlx::query!(
                "INSERT INTO spell_preset_component (spell_preset_uuid, component_name) VALUES ($1, $2)",
                id, component
            )
            .execute(&mut **tx)
            .await?;
        }

        for class_item in &spell_preset.class {
            sqlx::query!(
                "INSERT INTO spell_preset_class (spell_preset_uuid, spell_preset_class) VALUES ($1, $2)",
                id, class_item
            )
            .execute(&mut **tx)
            .await?;
        }

        for subclass_item in &spell_preset.sub_class {
            sqlx::query!(
                "INSERT INTO spell_preset_subclass (spell_preset_uuid, spell_preset_subclass) VALUES ($1, $2)",
                id, subclass_item
            )
            .execute(&mut **tx)
            .await?;
        }

        for species_item in &spell_preset.species {
            sqlx::query!(
                "INSERT INTO spell_preset_species (spell_preset_uuid, spell_preset_species) VALUES ($1, $2)",
                id, species_item
            )
            .execute(&mut **tx)
            .await?;
        }

        for feat_item in &spell_preset.feats {
            sqlx::query!(
                "INSERT INTO spell_preset_feats (spell_preset_uuid, spell_preset_feats) VALUES ($1, $2)",
                id, feat_item
            )
            .execute(&mut **tx)
            .await?;
        }

        Ok(())
    }

    /// Retrieves all spell presets.
    pub async fn get_all_spell_presets(&self) -> Result<Vec<SpellPreset>, Error> {
        let results = sqlx::query!(
            r#"
            SELECT 
                sp.uuid, sp.name, sp.level, sp.description, sp.time, 
                sp.concentration, sp.duration, sp.range, sp.school, sp.condition_inflict, sp.saving_throw, sp.affects_creature_type, sp.damage_inflict, sp.spell_attack, sp.ritual, sp.damage_immune, sp.ability_check, sp.damage_resist, sp.condition_immune, sp.higher_level,
                COALESCE(array_agg(DISTINCT spc.component_name) FILTER (WHERE spc.component_name IS NOT NULL), '{}') as "components!",
                COALESCE(array_agg(DISTINCT spclass.spell_preset_class) FILTER (WHERE spclass.spell_preset_class IS NOT NULL), '{}') as "class!",
                COALESCE(array_agg(DISTINCT spsc.spell_preset_subclass) FILTER (WHERE spsc.spell_preset_subclass IS NOT NULL), '{}') as "sub_class!",
                COALESCE(array_agg(DISTINCT spsp.spell_preset_species) FILTER (WHERE spsp.spell_preset_species IS NOT NULL), '{}') as "species!",
                COALESCE(array_agg(DISTINCT spf.spell_preset_feats) FILTER (WHERE spf.spell_preset_feats IS NOT NULL), '{}') as "feats!"
            FROM spell_preset sp
            LEFT JOIN spell_preset_component spc ON sp.uuid = spc.spell_preset_uuid
            LEFT JOIN spell_preset_class spclass ON sp.uuid = spclass.spell_preset_uuid
            LEFT JOIN spell_preset_subclass spsc ON sp.uuid = spsc.spell_preset_uuid
            LEFT JOIN spell_preset_species spsp ON sp.uuid = spsp.spell_preset_uuid
            LEFT JOIN spell_preset_feats spf ON sp.uuid = spf.spell_preset_uuid
            GROUP BY sp.uuid
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let spells = results
            .into_iter()
            .map(|result| SpellPreset {
                uuid: result.uuid,
                name: result.name,
                level: result.level,
                description: result.description,
                time: result.time,
                components: result.components,
                concentration: result.concentration,
                duration: result.duration,
                school: result.school,
                range: result.range,
                class: result.class,
                sub_class: result.sub_class,
                species: result.species,
                feats: result.feats,
                condition_inflict: result.condition_inflict,
                saving_throw: result.saving_throw,
                affects_creature_type: result.affects_creature_type,
                damage_inflict: result.damage_inflict,
                spell_attack: result.spell_attack,
                ritual: result.ritual,
                damage_immune: result.damage_immune,
                ability_check: result.ability_check,
                damage_resist: result.damage_resist,
                condition_immune: result.condition_immune,
                higher_level: result.higher_level,
            })
            .collect();

        Ok(spells)
    }

    /// Retrieves a spell preset by its UUID, or `None` if it does not exist.
    pub async fn find_spell_preset(&self, spell_preset_uuid: &str) -> Result<Option<SpellPreset>, Error> {
        match self.get_spell_preset(spell_preset_uuid).await {
            Ok(preset) => Ok(Some(preset)),
            Err(Error::RowNotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Deletes a spell preset by UUID. The preset is removed from all spell lists and
    /// all concentrations on it are ended.
    ///
    /// Returns the UUIDs of the users whose concentration was ended.
    pub async fn delete_spell_preset(&self, uuid: &str) -> Result<Vec<String>, Error> {
        let mut tx = self.pool.begin().await?;

        // Would also be removed by ON DELETE CASCADE, but the affected users are needed.
        let ended_concentrations = sqlx::query_scalar!(
            "DELETE FROM concentration WHERE spell_preset_uuid = $1 RETURNING owner_uuid",
            uuid
        )
        .fetch_all(&mut *tx)
        .await?;

        sqlx::query!("DELETE FROM spell_preset WHERE uuid = $1", uuid)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(ended_concentrations)
    }
}