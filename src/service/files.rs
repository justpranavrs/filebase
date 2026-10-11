// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use tracing::instrument;
use uuid::Uuid;

use crate::{
    models::{
        File, Folder, FolderBreadcrumb, NewFile, NewFolder, ROOT_FOLDER_ID, UpdateFile,
        UpdateFolder,
    },
    queries::{FileDB, FolderDB},
    service::Error,
};

// Contains the parent folder, its ancestors, subfolders, and files.
pub struct Directory {
    pub parent: Folder,
    pub ancestors: Vec<FolderBreadcrumb>,
    pub folders: Vec<Folder>,
    pub files: Vec<File>,
}

#[derive(Clone)]
pub struct FileService {
    file_db: FileDB,
    folder_db: FolderDB,
}

impl FileService {
    // Creates a new FileService instance with file and folder database repositories.
    pub fn new(file_db: FileDB, folder_db: FolderDB) -> Self {
        Self { file_db, folder_db }
    }

    // Persists a new file record via the database repository.
    #[instrument(skip(self))]
    pub async fn create_file(&self, file: NewFile) -> Result<File, Error> {
        Ok(self.file_db.create_file(file).await?)
    }

    // Marks a file as successfully uploaded via the database repository.
    #[instrument(skip(self))]
    pub async fn mark_uploaded(&self, key: Uuid) -> Result<File, Error> {
        self.file_db
            .mark_uploaded(key)
            .await?
            .ok_or(Error::NotFound)
    }

    // Persists a new folder record via the database repository.
    #[instrument(skip(self))]
    pub async fn create_folder(&self, folder: NewFolder) -> Result<Folder, Error> {
        Ok(self.folder_db.create_folder(folder).await?)
    }

    // Fetches the directory structure including parent folder, subfolders, files, and ancestors.
    #[instrument(skip(self))]
    pub async fn folder_items(&self, folder_id: i64) -> Result<Directory, Error> {
        let parent = self
            .folder_db
            .folder_by_id(folder_id)
            .await?
            .ok_or(Error::NotFound)?;

        let ancestors = self.ancestors(folder_id).await?;
        let files = self.file_db.files_by_folder_id(folder_id).await?;
        let folders = self.folder_db.folders_by_folder_id(folder_id).await?;

        Ok(Directory {
            parent,
            ancestors,
            folders,
            files,
        })
    }

    // Retrieves folder metadata by its primary key ID or returns a not found error.
    #[instrument(skip(self))]
    pub async fn folder_by_id(&self, id: i64) -> Result<Folder, Error> {
        self.folder_db
            .folder_by_id(id)
            .await?
            .ok_or(Error::NotFound)
    }

    // Retrieves ancestor folders in root-first order excluding the current folder.
    #[instrument(skip(self))]
    pub async fn ancestors(&self, folder_id: i64) -> Result<Vec<FolderBreadcrumb>, Error> {
        if folder_id == ROOT_FOLDER_ID {
            return Ok(vec![]);
        }

        let ancestors = self.folder_db.ancestors_by_folder_id(folder_id).await?;
        Ok(ancestors
            .into_iter()
            .filter(|a| a.id != folder_id)
            .collect())
    }

    // Retrieves destination folder info, ancestors, and direct subfolders for the move navigator.
    #[instrument(skip(self))]
    pub async fn move_navigator(
        &self,
        folder_id: i64,
    ) -> Result<(Folder, Vec<FolderBreadcrumb>, Vec<Folder>), Error> {
        let current_folder = self
            .folder_db
            .folder_by_id(folder_id)
            .await?
            .ok_or(Error::NotFound)?;
        let ancestors = self.folder_db.ancestors_by_folder_id(folder_id).await?;
        let subfolders = self.folder_db.folders_by_folder_id(folder_id).await?;
        Ok((current_folder, ancestors, subfolders))
    }

    // Retrieves file metadata by its storage object key or returns a not found error.
    #[instrument(skip(self))]
    pub async fn file_by_key(&self, object_key: Uuid) -> Result<File, Error> {
        self.file_db
            .file_by_key(object_key)
            .await?
            .ok_or(Error::NotFound)
    }

    // Updates a file's metadata record by its storage object key.
    #[instrument(skip(self))]
    pub async fn update_file(&self, key: Uuid, update: UpdateFile) -> Result<File, Error> {
        if let Some(target_folder_id) = update.folder_id {
            let current = self
                .file_db
                .file_by_key(key)
                .await?
                .ok_or(Error::NotFound)?;
            if current.folder_id == target_folder_id {
                return Err(Error::InvalidOperation(
                    "file is already in this directory".into(),
                ));
            }
        }
        self.file_db
            .update_file(key, update)
            .await?
            .ok_or(Error::NotFound)
    }

    // Deletes a file's metadata record by its storage object key.
    #[instrument(skip(self))]
    pub async fn delete_file(&self, key: Uuid) -> Result<File, Error> {
        self.file_db.delete_file(key).await?.ok_or(Error::NotFound)
    }

    // Updates a folder's metadata record by its primary key ID.
    #[instrument(skip(self))]
    pub async fn update_folder(&self, id: i64, update: UpdateFolder) -> Result<Folder, Error> {
        if let Some(target_parent_id) = update.parent_id {
            if target_parent_id == id {
                return Err(Error::InvalidOperation(
                    "cannot move a folder into itself".into(),
                ));
            }
            let current = self
                .folder_db
                .folder_by_id(id)
                .await?
                .ok_or(Error::NotFound)?;
            let current_parent_id = current.parent_id.unwrap_or(ROOT_FOLDER_ID);
            if current_parent_id == target_parent_id {
                return Err(Error::InvalidOperation(
                    "folder is already in this directory".into(),
                ));
            }
            if self
                .folder_db
                .would_create_cycle(id, target_parent_id)
                .await?
            {
                return Err(Error::InvalidOperation(
                    "cannot move a folder into itself or any of its subfolders".into(),
                ));
            }
        }
        self.folder_db
            .update_folder(id, update)
            .await?
            .ok_or(Error::NotFound)
    }
}
