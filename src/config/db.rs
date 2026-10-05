// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use serde::Deserialize;

// Represents the configuration for the database connection.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub timeout_seconds: u64,
}

// Provides default values for the DatabaseConfig struct.
impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite:/var/lib/filebase/data/filebase.db".into(),
            max_connections: 10,
            timeout_seconds: 30,
        }
    }
}
