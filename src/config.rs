// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

mod db;
mod log;
mod server;

pub use db::DatabaseConfig;
pub use log::LogConfig;
pub use server::ServerConfig;

// Represents the configuration for the application.
#[derive(Debug, Clone, Default)]
pub struct Config {
    pub db: DatabaseConfig,
    pub log: LogConfig,
    pub server: ServerConfig,
}

// Loads the configuration from environment variables and returns a Config instance.
pub fn load() -> Result<Config, envy::Error> {
    // Load environment variables from a .env file if it exists
    dotenvy::dotenv().ok();
    Ok(Config {
        db: envy::prefixed("DATABASE_").from_env()?,
        log: envy::prefixed("LOG_").from_env()?,
        server: envy::prefixed("SERVER_").from_env()?,
    })
}
