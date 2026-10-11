// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use std::path::{Path, PathBuf};

use tokio::{fs, io::AsyncWriteExt};
use uuid::Uuid;

use crate::{config::StorageConfig, service::Error};

#[derive(Clone)]
pub struct StorageService {
    cfg: StorageConfig,
}

impl StorageService {
    // Resolves the local filesystem path for a storage object key.
    fn resolve_path(&self, key: &Uuid) -> PathBuf {
        Path::new(&self.cfg.dir).join(key.to_string())
    }

    // Creates a new StorageService instance with the given storage configuration.
    pub fn new(cfg: StorageConfig) -> Self {
        Self { cfg }
    }

    // Reads all binary data for a stored file by its object key.
    pub async fn read_file(&self, key: &Uuid) -> Result<Vec<u8>, Error> {
        let path = self.resolve_path(key);
        fs::read(&path).await.map_err(Into::into)
    }

    // Writes binary data to disk under a newly generated UUID object key.
    pub async fn save_file(&self, data: &[u8]) -> Result<Uuid, Error> {
        let key = uuid::Uuid::now_v7();
        let path = self.resolve_path(&key);

        let mut file = fs::File::create(&path).await?;
        file.write_all(data).await?;
        file.flush().await?;
        Ok(key)
    }

    // Deletes the file from disk using its storage object key.
    pub async fn delete_file(&self, key: &Uuid) -> Result<(), Error> {
        let path = self.resolve_path(key);
        match fs::remove_file(&path).await {
            Ok(_) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
