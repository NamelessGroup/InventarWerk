use crate::{Repository, UserDb};
use crate::model_zauberwerk::Concentration;
use sqlx::Error;

pub struct ConcentrationRepository {
    db: UserDb,
}

impl ConcentrationRepository {
    pub fn new(db: UserDb) -> Self {
        Self { db }
    }

    /// Retrieves the concentration of a specific owner.
    /// `spell_preset_uuid` is `None` if the owner is not concentrating.
    pub async fn get_concentration(&self, owner_uuid: &str) -> Result<Concentration, Error> {
        let spell_preset_uuid = sqlx::query_scalar!(
            "SELECT spell_preset_uuid FROM concentration WHERE owner_uuid = $1",
            owner_uuid
        )
        .fetch_optional(&mut *self.db.acquire().await?)
        .await?;

        Ok(Concentration {
            owner_uuid: owner_uuid.to_string(),
            spell_preset_uuid,
        })
    }

    /// Ends the concentration of the owner.
    ///
    /// Returns the UUID of the spell preset the owner was concentrating on,
    /// or `None` if the owner was not concentrating.
    pub async fn remove_concentration(&self, owner_uuid: &str) -> Result<Option<String>, Error> {
        let ended = sqlx::query_scalar!(
            "DELETE FROM concentration WHERE owner_uuid = $1 RETURNING spell_preset_uuid",
            owner_uuid
        )
        .fetch_optional(&mut *self.db.acquire().await?)
        .await?;
        Ok(ended)
    }

    /// Retrieves all users that are currently concentrating.
    pub async fn get_all_concentrations(&self) -> Result<Vec<Concentration>, Error> {
        let concentrations = sqlx::query_as!(
            Concentration,
            r#"SELECT owner_uuid, spell_preset_uuid AS "spell_preset_uuid?" FROM concentration"#
        )
        .fetch_all(&mut *self.db.acquire().await?)
        .await?;

        Ok(concentrations)
    }
}

impl Repository for ConcentrationRepository {
    fn from_db(db: UserDb) -> Self {
        Self::new(db)
    }
}
