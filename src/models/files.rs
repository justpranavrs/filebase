// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use sqlx::types::chrono;
use uuid::Uuid;

// Metadata for a file stored in the object storage.
#[derive(Debug)]
pub struct File {
    pub id: i64,
    pub folder_id: i64,
    pub object_key: Uuid,
    pub name: String,
    pub size: i64,
    pub content_type: Option<String>,
    pub checksum: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

// Metadata for creating a new file in the database.
#[derive(Debug)]
pub struct NewFile {
    pub folder_id: i64,
    pub object_key: Uuid,
    pub name: String,
    pub size: i64,
    pub content_type: Option<String>,
    pub checksum: Option<String>,
}
