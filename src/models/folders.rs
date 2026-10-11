// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use serde::Deserialize;
use sqlx::types::chrono;

pub const ROOT_FOLDER_ID: i64 = 1;

// A folder in the virtual filesystem of the app.
#[derive(Debug)]
pub struct Folder {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

// Represents an ancestor folder in the breadcrumb trail.
#[derive(Debug, Clone)]
pub struct FolderBreadcrumb {
    pub id: i64,
    pub name: String,
}

// Metadata for creating a new folder in the database.
#[derive(Debug, Deserialize)]
pub struct NewFolder {
    pub parent_id: i64,
    pub name: String,
}

// Metadata for updating an existing folder's name or parent folder location.
#[derive(Debug, Deserialize)]
pub struct UpdateFolder {
    pub parent_id: Option<i64>,
    pub name: Option<String>,
}
