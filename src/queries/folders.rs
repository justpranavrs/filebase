// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use sqlx::{SqlitePool, query_as};
use tracing::instrument;

use crate::models::{Folder, NewFolder};

#[derive(Clone)]
pub struct FolderDB {
    pool: SqlitePool,
}

impl FolderDB {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    #[instrument(skip(self), err)]
    pub async fn create_folder(&self, folder: NewFolder) -> sqlx::Result<Folder> {
        query_as!(
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

    #[instrument(skip(self), err)]
    pub async fn folder_by_id(&self, id: i64) -> sqlx::Result<Option<Folder>> {
        query_as!(
            Folder,
            r#"
                SELECT * FROM folders
                WHERE id IS ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    #[instrument(skip(self), err)]
    pub async fn folders_by_folder_id(&self, parent_id: i64) -> sqlx::Result<Vec<Folder>> {
        query_as!(
            Folder,
            r#"
                SELECT * FROM folders
                WHERE parent_id IS ?
            "#,
            parent_id
        )
        .fetch_all(&self.pool)
        .await
    }
}
