// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use sqlx::SqlitePool;
use tracing::instrument;
use uuid::Uuid;

use crate::models::{File, NewFile, UpdateFile};

#[derive(Clone)]
pub struct FileDB {
    pool: SqlitePool,
}

impl FileDB {
    // Creates a new FileDB repository instance with the database connection pool.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // Inserts a new file metadata record into the database.
    #[instrument(skip(self), err)]
    pub async fn create_file(&self, file: NewFile) -> sqlx::Result<File> {
        sqlx::query_as!(
            File,
            r#"
                INSERT INTO files (folder_id, object_key, name, size, content_type, checksum)
                VALUES (?, ?, ?, ?, ?, ?)
                RETURNING
                    id AS "id!", folder_id, object_key, name,
                    size, content_type, checksum,
                    created_at, updated_at
            "#,
            file.folder_id,
            file.object_key,
            file.name,
            file.size,
            file.content_type,
            file.checksum
        )
        .fetch_one(&self.pool)
        .await
    }

    // Retrieves a file record by its unique storage object key.
    #[instrument(skip(self), err)]
    pub async fn file_by_key(&self, object_key: Uuid) -> sqlx::Result<Option<File>> {
        sqlx::query_as!(
            File,
            r#"
                SELECT
                    id, folder_id, object_key, name,
                    size, content_type, checksum,
                    created_at, updated_at
                FROM files 
                WHERE object_key is ?
            "#,
            object_key
        )
        .fetch_optional(&self.pool)
        .await
    }

    // Retrieves all file records contained within a specific folder.
    #[instrument(skip(self), err)]
    pub async fn files_by_folder_id(&self, folder_id: i64) -> sqlx::Result<Vec<File>> {
        sqlx::query_as!(
            File,
            r#"
                SELECT 
                    id, folder_id, object_key, name,
                    size, content_type, checksum,
                    created_at, updated_at
                FROM files 
                WHERE folder_id IS ?
            "#,
            folder_id
        )
        .fetch_all(&self.pool)
        .await
    }

    // Updates a file's name and/or parent folder by its storage object key.
    #[instrument(skip(self), err)]
    pub async fn update_file(&self, key: Uuid, update: UpdateFile) -> sqlx::Result<Option<File>> {
        let name = update.name.filter(|s| !s.trim().is_empty());
        sqlx::query_as!(
            File,
            r#"
                UPDATE files
                SET
                    folder_id = COALESCE(?, folder_id),
                    name = COALESCE(?, name),
                    updated_at = datetime('now')
                WHERE object_key IS ?
                RETURNING
                    id AS "id!", folder_id, object_key, name,
                    size, content_type, checksum,
                    created_at, updated_at
            "#,
            update.folder_id,
            name,
            key
        )
        .fetch_optional(&self.pool)
        .await
    }

    // Deletes a file record by its storage object key and returns the deleted record.
    #[instrument(skip(self), err)]
    pub async fn delete_file(&self, key: Uuid) -> sqlx::Result<Option<File>> {
        sqlx::query_as!(
            File,
            r#"
                DELETE FROM files
                WHERE object_key = ?
                RETURNING
                    id AS "id!", folder_id, object_key, name,
                    size, content_type, checksum,
                    created_at, updated_at
            "#,
            key
        )
        .fetch_optional(&self.pool)
        .await
    }
}
