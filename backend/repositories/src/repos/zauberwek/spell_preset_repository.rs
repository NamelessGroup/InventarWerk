use crate::model_zauberwerk::SpellPreset;
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

    /// Retrieves a spell preset by its UUID.
    pub async fn get_spell_preset(&self, spell_preset_uuid: &str) -> Result<SpellPreset, Error> {
        let result = sqlx::query!(
            r#"
            SELECT 
                sp.uuid, sp.name, sp.level, sp.description, sp.time, 
                sp.concentration, sp.duration, sp.range,
                COALESCE(array_agg(DISTINCT spc.component_name) FILTER (WHERE spc.component_name IS NOT NULL), '{}') as "components!",
                COALESCE(array_agg(DISTINCT sps.spell_preset_school) FILTER (WHERE sps.spell_preset_school IS NOT NULL), '{}') as "school!",
                COALESCE(array_agg(DISTINCT spclass.spell_preset_class) FILTER (WHERE spclass.spell_preset_class IS NOT NULL), '{}') as "class!",
                COALESCE(array_agg(DISTINCT spsc.spell_preset_subclass) FILTER (WHERE spsc.spell_preset_subclass IS NOT NULL), '{}') as "sub_class!",
                COALESCE(array_agg(DISTINCT spsp.spell_preset_species) FILTER (WHERE spsp.spell_preset_species IS NOT NULL), '{}') as "species!",
                COALESCE(array_agg(DISTINCT spf.spell_preset_feats) FILTER (WHERE spf.spell_preset_feats IS NOT NULL), '{}') as "feats!"
            FROM spell_preset sp
            LEFT JOIN spell_preset_component spc ON sp.uuid = spc.spell_preset_uuid
            LEFT JOIN spell_preset_school sps ON sp.uuid = sps.spell_preset_uuid
            LEFT JOIN spell_preset_class spclass ON sp.uuid = spclass.spell_preset_uuid
            LEFT JOIN spell_preset_subclass spsc ON sp.uuid = spsc.spell_preset_uuid
            LEFT JOIN spell_preset_species spsp ON sp.uuid = spsp.spell_preset_uuid
            LEFT JOIN spell_preset_feats spf ON sp.uuid = spf.spell_preset_uuid
            WHERE sp.uuid = $1
            GROUP BY sp.uuid, sp.name, sp.level, sp.description, sp.time, 
                     sp.concentration, sp.duration, sp.range
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
            concentration: result.concentration != 0,
            duration: result.duration,
            school: result.school,
            range: vec![result.range],
            class: result.class,
            sub_class: result.sub_class,
            species: result.species,
            feats: result.feats,
        })
    }

    /// Creates a new spell preset.
    pub async fn create_spell_preset(&self, spell_preset: SpellPreset) -> Result<String, Error> {
        let id = if spell_preset.uuid.is_empty() {
            Uuid::new_v4().to_string()
        } else {
            spell_preset.uuid.clone()
        };

        // Insert the main spell preset
        let range_value = spell_preset.range.first().cloned().unwrap_or_else(String::new);

        sqlx::query!(
            "INSERT INTO spell_preset (uuid, name, level, description, time, concentration, duration, range)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            id,
            spell_preset.name,
            spell_preset.level,
            spell_preset.description,
            spell_preset.time,
            if spell_preset.concentration { 1 } else { 0 },
            spell_preset.duration,
            range_value
        )
        .execute(&self.pool)
        .await?;

        // Insert components
        for component in &spell_preset.components {
            sqlx::query!(
                "INSERT INTO spell_preset_component (spell_preset_uuid, component_name) VALUES ($1, $2)",
                id, component
            )
            .execute(&self.pool)
            .await?;
        }

        // Insert schools
        for school_item in &spell_preset.school {
            sqlx::query!(
                "INSERT INTO spell_preset_school (spell_preset_uuid, spell_preset_school) VALUES ($1, $2)",
                id, school_item
            )
            .execute(&self.pool)
            .await?;
        }

        // Insert classes
        for class_item in &spell_preset.class {
            sqlx::query!(
                "INSERT INTO spell_preset_class (spell_preset_uuid, spell_preset_class) VALUES ($1, $2)",
                id, class_item
            )
            .execute(&self.pool)
            .await?;
        }

        // Insert subclasses
        for subclass_item in &spell_preset.sub_class {
            sqlx::query!(
                "INSERT INTO spell_preset_subclass (spell_preset_uuid, spell_preset_subclass) VALUES ($1, $2)",
                id, subclass_item
            )
            .execute(&self.pool)
            .await?;
        }

        // Insert species
        for species_item in &spell_preset.species {
            sqlx::query!(
                "INSERT INTO spell_preset_species (spell_preset_uuid, spell_preset_species) VALUES ($1, $2)",
                id, species_item
            )
            .execute(&self.pool)
            .await?;
        }

        // Insert feats
        for feat_item in &spell_preset.feats {
            sqlx::query!(
                "INSERT INTO spell_preset_feats (spell_preset_uuid, spell_preset_feats) VALUES ($1, $2)",
                id, feat_item
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(id)
    }

    /// Modifies an existing spell preset by deleting and recreating it.
    pub async fn modify_spell_preset(&self, new_spell_preset: SpellPreset) -> Result<(), Error> {
        // Delete the existing spell preset (CASCADE will handle related tables)
        sqlx::query!(
            "DELETE FROM spell_preset WHERE uuid = $1",
            new_spell_preset.uuid
        )
        .execute(&self.pool)
        .await?;

        // Create the new version
        self.create_spell_preset(new_spell_preset).await?;

        Ok(())
    }

    /// Retrieves all spell presets.
    pub async fn get_all_spell_presets(&self) -> Result<Vec<SpellPreset>, Error> {
        let results = sqlx::query!(
            r#"
            SELECT 
                sp.uuid, sp.name, sp.level, sp.description, sp.time, 
                sp.concentration, sp.duration, sp.range,
                COALESCE(array_agg(DISTINCT spc.component_name) FILTER (WHERE spc.component_name IS NOT NULL), '{}') as "components!",
                COALESCE(array_agg(DISTINCT sps.spell_preset_school) FILTER (WHERE sps.spell_preset_school IS NOT NULL), '{}') as "school!",
                COALESCE(array_agg(DISTINCT spclass.spell_preset_class) FILTER (WHERE spclass.spell_preset_class IS NOT NULL), '{}') as "class!",
                COALESCE(array_agg(DISTINCT spsc.spell_preset_subclass) FILTER (WHERE spsc.spell_preset_subclass IS NOT NULL), '{}') as "sub_class!",
                COALESCE(array_agg(DISTINCT spsp.spell_preset_species) FILTER (WHERE spsp.spell_preset_species IS NOT NULL), '{}') as "species!",
                COALESCE(array_agg(DISTINCT spf.spell_preset_feats) FILTER (WHERE spf.spell_preset_feats IS NOT NULL), '{}') as "feats!"
            FROM spell_preset sp
            LEFT JOIN spell_preset_component spc ON sp.uuid = spc.spell_preset_uuid
            LEFT JOIN spell_preset_school sps ON sp.uuid = sps.spell_preset_uuid
            LEFT JOIN spell_preset_class spclass ON sp.uuid = spclass.spell_preset_uuid
            LEFT JOIN spell_preset_subclass spsc ON sp.uuid = spsc.spell_preset_uuid
            LEFT JOIN spell_preset_species spsp ON sp.uuid = spsp.spell_preset_uuid
            LEFT JOIN spell_preset_feats spf ON sp.uuid = spf.spell_preset_uuid
            GROUP BY sp.uuid, sp.name, sp.level, sp.description, sp.time, 
                     sp.concentration, sp.duration, sp.range
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
                concentration: result.concentration != 0,
                duration: result.duration,
                school: result.school,
                range: vec![result.range],
                class: result.class,
                sub_class: result.sub_class,
                species: result.species,
                feats: result.feats,
            })
            .collect();

        Ok(spells)
    }

    /// Deletes a spell preset by UUID.
    pub async fn delete_spell_preset(&self, uuid: &str) -> Result<(), Error> {
        sqlx::query!("DELETE FROM spell_preset WHERE uuid = $1", uuid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}