use crate::model_zauberwerk::{SpellList, SpellListEntry};
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
            spell_lists.push(self.build_spell_list(list.uuid, list.name, list.owner_uuid).await?);
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
            spell_lists.push(self.build_spell_list(list.uuid, list.name, list.owner_uuid).await?);
        }

        Ok(spell_lists)
    }

    /// Checks whether the spell preset is contained in at least one spell list of the owner.
    pub async fn owner_has_spell(&self, owner_uuid: &str, spell_preset_uuid: &str) -> Result<bool, Error> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(
                SELECT 1 FROM spell_list_entry e
                JOIN spell_list l ON l.uuid = e.spell_list_uuid
                WHERE l.owner_uuid = $1 AND e.spell_preset_uuid = $2
            ) AS "exists!""#,
            owner_uuid,
            spell_preset_uuid
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(exists)
    }

    /// Retrieves a specific spell list by UUID.
    pub async fn get_spell_list(&self, spell_list_uuid: &str) -> Result<SpellList, Error> {
        let list = sqlx::query!(
            "SELECT uuid, name, owner_uuid FROM spell_list WHERE uuid = $1",
            spell_list_uuid
        )
        .fetch_one(&self.pool)
        .await?;

        self.build_spell_list(list.uuid, list.name, list.owner_uuid).await
    }

    /// Builds a `SpellList` with its entries, ordered by `sorting`.
    async fn build_spell_list(&self, uuid: String, name: String, owner_uuid: String) -> Result<SpellList, Error> {
        let entries = sqlx::query_as!(
            SpellListEntry,
            "SELECT spell_preset_uuid, sorting FROM spell_list_entry
             WHERE spell_list_uuid = $1
             ORDER BY sorting, spell_preset_uuid",
            uuid
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(SpellList {
            uuid,
            name,
            owner_uuid,
            entries,
        })
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

    /// Adds a spell preset to a spell list. Does nothing if the preset is already in the list.
    ///
    /// If `sorting` is `None`, the spell is appended at the end of the list.
    pub async fn add_preset_to_list(&self, spell_list_uuid: &str, spell_preset_uuid: &str, sorting: Option<i32>) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO spell_list_entry (spell_preset_uuid, spell_list_uuid, sorting)
             VALUES ($1, $2, COALESCE(
                 $3,
                 (SELECT COALESCE(MAX(sorting) + 1, 0) FROM spell_list_entry WHERE spell_list_uuid = $2)
             ))
             ON CONFLICT DO NOTHING",
            spell_preset_uuid,
            spell_list_uuid,
            sorting
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Sets the position of a spell preset in a spell list.
    ///
    /// Returns `false` if the preset is not in the list.
    pub async fn set_sorting(&self, spell_list_uuid: &str, spell_preset_uuid: &str, sorting: i32) -> Result<bool, Error> {
        let updated = sqlx::query!(
            "UPDATE spell_list_entry SET sorting = $3
             WHERE spell_list_uuid = $1 AND spell_preset_uuid = $2",
            spell_list_uuid,
            spell_preset_uuid,
            sorting
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        Ok(updated > 0)
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