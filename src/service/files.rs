// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use super::Error;

use tracing::instrument;

use crate::{models::{File, Folder}, queries::{FileDB, FolderDB}};

// Shows the Items inside of a Directory.
pub struct FolderItems {
    folders: Vec<Folder>,
    files: Vec<File>,
}

#[derive(Clone)]
pub struct FileService {
    db: FileDB,
    folder_db: FolderDB,
}

impl FileService {
    pub fn new(db: FileDB, folder_db: FolderDB) -> Self {
        Self { db, folder_db }
    }

    #[instrument(skip(self))]
    pub async fn folder_items(&self, folder_id: i64) -> Result<FolderItems, Error> {
        let files = self.db.files(folder_id).await?;
        let folders = self.folder_db.folders(folder_id).await?;
        Ok(FolderItems { folders, files })
    }
}