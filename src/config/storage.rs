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
    // Provides default configuration values for file storage.
    fn default() -> Self {
        Self {
            dir: "./.storage".into(),
        }
    }
}
