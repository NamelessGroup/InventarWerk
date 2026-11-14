use crate::model_zauberwerk::SpellAttackModifier;
use sqlx::{Error, PgPool};

pub struct SpellAttackModifierRepository {
    pool: PgPool,
}

impl SpellAttackModifierRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Retrieves the spell attack modifier of a specific owner.
    /// `modifier` is `None` if no modifier was set yet.
    pub async fn get_spell_attack_modifier(&self, owner_uuid: &str) -> Result<SpellAttackModifier, Error> {
        let modifier = sqlx::query_scalar!(
            "SELECT modifier FROM spell_attack_modifier WHERE owner_uuid = $1",
            owner_uuid
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(SpellAttackModifier {
            owner_uuid: owner_uuid.to_string(),
            modifier,
        })
    }

    /// Sets the spell attack modifier of the owner, replacing any previous value.
    pub async fn set_spell_attack_modifier(&self, owner_uuid: &str, modifier: i32) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO spell_attack_modifier (owner_uuid, modifier)
             VALUES ($1, $2)
             ON CONFLICT (owner_uuid)
             DO UPDATE SET modifier = $2",
            owner_uuid,
            modifier
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Retrieves the spell attack modifiers of all users that have one set.
    pub async fn get_all_spell_attack_modifiers(&self) -> Result<Vec<SpellAttackModifier>, Error> {
        let modifiers = sqlx::query_as!(
            SpellAttackModifier,
            r#"SELECT owner_uuid, modifier AS "modifier?" FROM spell_attack_modifier"#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(modifiers)
    }
}
