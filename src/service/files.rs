// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use super::Error;

use tracing::instrument;

use crate::{
    models::{File, Folder, NewFolder},
    queries::{FileDB, FolderDB},
};

// Shows the items inside of a Directory.
pub struct FolderItems {
    pub folders: Vec<Folder>,
    pub files: Vec<File>,
}

// Contains the parent folder and items of that folder.
pub struct Directory {
    pub parent: Folder,
    pub items: FolderItems,
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
    pub async fn create_folder(&self, folder: NewFolder) -> Result<Folder, Error> {
        Ok(self.folder_db.create_folder(folder).await?)
    }

    #[instrument(skip(self))]
    pub async fn folder_items(&self, folder_id: i64) -> Result<Directory, Error> {
        let parent = self
            .folder_db
            .folder_by_id(folder_id)
            .await?
            .ok_or(Error::NotFound)?;

        let files = self.db.files_by_folder_id(folder_id).await?;
        let folders = self.folder_db.folders_by_folder_id(folder_id).await?;
        Ok(Directory {
            parent,
            items: FolderItems { folders, files },
        })
    }
}
