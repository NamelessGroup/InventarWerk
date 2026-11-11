use crate::model_zauberwerk::Concentration;
use sqlx::{Error, PgPool};

pub struct ConcentrationRepository {
    pool: PgPool,
}

impl ConcentrationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Retrieves the concentration of a specific owner.
    /// `spell_preset_uuid` is `None` if the owner is not concentrating.
    pub async fn get_concentration(&self, owner_uuid: &str) -> Result<Concentration, Error> {
        let spell_preset_uuid = sqlx::query_scalar!(
            "SELECT spell_preset_uuid FROM concentration WHERE owner_uuid = $1",
            owner_uuid
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(Concentration {
            owner_uuid: owner_uuid.to_string(),
            spell_preset_uuid,
        })
    }

    /// Sets the spell the owner is concentrating on, replacing any previous concentration.
    pub async fn set_concentration(&self, owner_uuid: &str, spell_preset_uuid: &str) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO concentration (owner_uuid, spell_preset_uuid)
             VALUES ($1, $2)
             ON CONFLICT (owner_uuid)
             DO UPDATE SET spell_preset_uuid = $2",
            owner_uuid,
            spell_preset_uuid
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Ends the concentration of the owner. Does nothing if the owner is not concentrating.
    pub async fn remove_concentration(&self, owner_uuid: &str) -> Result<(), Error> {
        sqlx::query!(
            "DELETE FROM concentration WHERE owner_uuid = $1",
            owner_uuid
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Retrieves all users that are currently concentrating.
    pub async fn get_all_concentrations(&self) -> Result<Vec<Concentration>, Error> {
        let concentrations = sqlx::query_as!(
            Concentration,
            r#"SELECT owner_uuid, spell_preset_uuid AS "spell_preset_uuid?" FROM concentration"#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(concentrations)
    }
}
