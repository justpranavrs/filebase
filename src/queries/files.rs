// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use sqlx::{SqlitePool, query_as};
use tracing::instrument;

use crate::models::File;

#[derive(Clone)]
pub struct FileDB {
    pool: SqlitePool,
}

impl FileDB {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    #[instrument(skip(self), err)]
    pub async fn files(&self, folder_id: i64) -> sqlx::Result<Vec<File>> {
        query_as!(
            File,
            r#"
                SELECT * FROM files 
                WHERE folder_id IS ?
            "#,
            folder_id
        )
        .fetch_all(&self.pool)
        .await
    }
}
