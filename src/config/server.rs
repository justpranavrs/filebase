// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use serde::Deserialize;

// Represents the configuration for the server.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub timeout_seconds: u64,
}

// Provides default values for the ServerConfig struct.
impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "filebase".into(),
            port: 3000,
            timeout_seconds: 30,
        }
    }
}
