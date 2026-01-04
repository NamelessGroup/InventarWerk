use crate::model_zauberwerk::Concentration;
use anyhow::{self, Result};
use sqlx::{Error, PgPool};

pub struct ConcentrationRepository {
    pool: PgPool,
}

impl ConcentrationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Retrieves the concentration value for a specific owner.
    pub async fn get_concentration(&self, owner_uuid: &str) -> Result<Concentration, Error> {
        let result = sqlx::query_as!(
            Concentration,
            "SELECT owner_uuid, concentration FROM concentration WHERE owner_uuid = $1",
            owner_uuid
        )
        .fetch_one(&self.pool)
        .await?;    
        Ok(result)
    }

    /// Sets the concentration value for a specific owner.
    pub async fn set_concentration(&self, owner_uuid: &str, concentration: i32) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO concentration (owner_uuid, concentration)
             VALUES ($1, $2)
             ON CONFLICT (owner_uuid) 
             DO UPDATE SET concentration = $2",
            owner_uuid,
            concentration
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Retrieves all concentration entries.
    pub async fn get_all_concentrations(&self) -> Result<Vec<Concentration>, Error> {
        let concentrations = sqlx::query_as!(
            Concentration,
            "SELECT owner_uuid, concentration FROM concentration"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(concentrations)
    }
}