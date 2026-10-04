// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use sqlx::{SqlitePool, query_as};
use tracing::instrument;

use crate::models::Folder;

#[derive(Clone)]
pub struct FolderDB {
    pool: SqlitePool,
}

impl FolderDB {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    #[instrument(skip(self), err)]
    pub async fn folders(&self, parent_id: i64) -> sqlx::Result<Vec<Folder>> {
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
