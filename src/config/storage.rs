// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use serde::Deserialize;

// Represents the configuration for the file storage.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct StorageConfig {
    pub dir: String,
}

// Provides default values for the StorageConfig struct.
impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            dir: "/var/lib/filebase/storage".into(),
        }
    }
}
