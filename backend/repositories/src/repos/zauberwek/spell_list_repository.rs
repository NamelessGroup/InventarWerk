use crate::model_zauberwerk::SpellList;
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct SpellListRepository {
    pool: PgPool,
}

impl SpellListRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Retrieves all spell lists for a specific owner.
    pub async fn get_spell_list_for_owner(&self, owner_uuid: &str) -> Result<Vec<SpellList>, Error> {
        let lists = sqlx::query!(
            "SELECT uuid, name, owner_uuid FROM spell_list WHERE owner_uuid = $1",
            owner_uuid
        )
        .fetch_all(&self.pool)
        .await?;

        let mut spell_lists = Vec::new();
        for list in lists {
            let spell_uuids = self.get_spell_uuids_in_list(&list.uuid).await?;
            spell_lists.push(SpellList {
                uuid: list.uuid,
                name: list.name,
                owner_uuid: list.owner_uuid,
                spell_uuids,
            });
        }

        Ok(spell_lists)
    }

    /// Retrieves all spell lists.
    pub async fn get_all_spell_lists(&self) -> Result<Vec<SpellList>, Error> {
        let lists = sqlx::query!("SELECT uuid, name, owner_uuid FROM spell_list")
            .fetch_all(&self.pool)
            .await?;

        let mut spell_lists = Vec::new();
        for list in lists {
            let spell_uuids = self.get_spell_uuids_in_list(&list.uuid).await?;
            spell_lists.push(SpellList {
                uuid: list.uuid,
                name: list.name,
                owner_uuid: list.owner_uuid,
                spell_uuids,
            });
        }

        Ok(spell_lists)
    }

    /// Retrieves a specific spell list by UUID.
    pub async fn get_spell_list(&self, spell_list_uuid: &str) -> Result<SpellList, Error> {
        let list = sqlx::query!(
            "SELECT uuid, name, owner_uuid FROM spell_list WHERE uuid = $1",
            spell_list_uuid
        )
        .fetch_one(&self.pool)
        .await?;

        let spell_uuids = self.get_spell_uuids_in_list(&list.uuid).await?;

        Ok(SpellList {
            uuid: list.uuid,
            name: list.name,
            owner_uuid: list.owner_uuid,
            spell_uuids,
        })
    }

    /// Helper method to get all spell UUIDs in a list.
    async fn get_spell_uuids_in_list(&self, spell_list_uuid: &str) -> Result<Vec<String>, Error> {
        let entries = sqlx::query!(
            "SELECT spell_preset_uuid FROM spell_list_entry WHERE spell_list_uuid = $1",
            spell_list_uuid
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(entries.into_iter().map(|e| e.spell_preset_uuid).collect())
    }

    /// Creates a new spell list.
    pub async fn create_spell_list(&self, owner_uuid: &str, name: &str) -> Result<String, Error> {
        let uuid = Uuid::new_v4().to_string();
        sqlx::query!(
            "INSERT INTO spell_list (uuid, owner_uuid, name) VALUES ($1, $2, $3)",
            uuid,
            owner_uuid,
            name
        )
        .execute(&self.pool)
        .await?;

        Ok(uuid)
    }

    /// Renames a spell list.
    pub async fn rename_spell_list(&self, spell_list_uuid: &str, new_name: &str) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE spell_list SET name = $2 WHERE uuid = $1",
            spell_list_uuid,
            new_name
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Adds a spell preset to a spell list.
    pub async fn add_preset_to_list(&self, spell_list_uuid: &str, spell_preset_uuid: &str) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO spell_list_entry (spell_preset_uuid, spell_list_uuid) 
             VALUES ($1, $2)
             ON CONFLICT DO NOTHING",
            spell_preset_uuid,
            spell_list_uuid
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Removes a spell preset from a spell list.
    pub async fn remove_preset_from_list(&self, spell_list_uuid: &str, spell_preset_uuid: &str) -> Result<(), Error> {
        sqlx::query!(
            "DELETE FROM spell_list_entry 
             WHERE spell_list_uuid = $1 AND spell_preset_uuid = $2",
            spell_list_uuid,
            spell_preset_uuid
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Deletes a spell list.
    pub async fn delete_spell_list(&self, spell_list_uuid: &str) -> Result<(), Error> {
        sqlx::query!("DELETE FROM spell_list WHERE uuid = $1", spell_list_uuid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}