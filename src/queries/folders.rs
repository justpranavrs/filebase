// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use sqlx::SqlitePool;
use tracing::instrument;

use crate::models::{Folder, FolderBreadcrumb, NewFolder, UpdateFolder};

#[derive(Clone)]
pub struct FolderDB {
    pool: SqlitePool,
}

impl FolderDB {
    // Creates a new FolderDB repository instance with the database connection pool.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // Inserts a new folder record into the database.
    #[instrument(skip(self), err)]
    pub async fn create_folder(&self, folder: NewFolder) -> sqlx::Result<Folder> {
        sqlx::query_as!(
            Folder,
            r#"
                INSERT INTO folders (parent_id, name)
                VALUES (?, ?)
                RETURNING id, parent_id, name, created_at, updated_at
            "#,
            folder.parent_id,
            folder.name
        )
        .fetch_one(&self.pool)
        .await
    }

    // Retrieves a folder record by its primary key identifier.
    #[instrument(skip(self), err)]
    pub async fn folder_by_id(&self, id: i64) -> sqlx::Result<Option<Folder>> {
        sqlx::query_as!(
            Folder,
            r#"
                SELECT 
                    id, parent_id, name, created_at, updated_at 
                FROM folders
                WHERE id IS ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    // Retrieves all subfolder records within a specific parent folder.
    #[instrument(skip(self), err)]
    pub async fn folders_by_folder_id(&self, parent_id: i64) -> sqlx::Result<Vec<Folder>> {
        sqlx::query_as!(
            Folder,
            r#"
                SELECT 
                    id, parent_id, name, created_at, updated_at 
                FROM folders
                WHERE parent_id IS ?
            "#,
            parent_id
        )
        .fetch_all(&self.pool)
        .await
    }

    // Updates a folder's name and/or parent folder by its primary key ID.
    #[instrument(skip(self), err)]
    pub async fn update_folder(
        &self,
        id: i64,
        update: UpdateFolder,
    ) -> sqlx::Result<Option<Folder>> {
        let name = update.name.filter(|s| !s.trim().is_empty());
        sqlx::query_as!(
            Folder,
            r#"
                UPDATE folders
                SET
                    parent_id = COALESCE(?, parent_id),
                    name = COALESCE(?, name),
                    updated_at = datetime('now')
                WHERE id IS ?
                RETURNING id, parent_id, name, created_at, updated_at
            "#,
            update.parent_id,
            name,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    // Retrieves the ordered ancestor chain from root down to the given folder.
    #[instrument(skip(self), err)]
    pub async fn ancestors_by_folder_id(
        &self,
        folder_id: i64,
    ) -> sqlx::Result<Vec<FolderBreadcrumb>> {
        sqlx::query_as!(
            FolderBreadcrumb,
            r#"
                WITH RECURSIVE ancestors(id, parent_id, name, depth) AS (
                    SELECT id, parent_id, name, 0 AS depth
                    FROM folders
                    WHERE id = ?

                    UNION ALL

                    SELECT f.id, f.parent_id, f.name, a.depth + 1
                    FROM folders f
                    JOIN ancestors a ON f.id = a.parent_id
                )
                SELECT id, name
                FROM ancestors
                ORDER BY depth DESC
            "#,
            folder_id
        )
        .fetch_all(&self.pool)
        .await
    }

    // Checks if moving `id` into `target_parent_id` would create a circular reference.
    #[instrument(skip(self), err)]
    pub async fn would_create_cycle(&self, id: i64, target_parent_id: i64) -> sqlx::Result<bool> {
        let is_cycle = sqlx::query_scalar!(
            r#"
                WITH RECURSIVE ancestors(id, parent_id) AS (
                    SELECT id, parent_id
                    FROM folders
                    WHERE id = ?

                    UNION ALL

                    SELECT f.id, f.parent_id
                    FROM folders f
                    JOIN ancestors a ON f.id = a.parent_id
                )
                SELECT EXISTS(SELECT 1 FROM ancestors WHERE id = ?) AS is_cycle
            "#,
            target_parent_id,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(is_cycle == 1)
    }
}
