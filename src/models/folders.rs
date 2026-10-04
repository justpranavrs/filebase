// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use sqlx::types::chrono;

// A folder in the virtual filesystem of the app.
#[derive(Debug)]
pub struct Folder {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

// Metadata for creating a new folder in the database.
#[derive(Debug)]
pub struct NewFolder {
    pub parent_id: i64,
    pub name: String,
}
