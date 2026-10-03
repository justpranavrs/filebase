// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
};
use std::str::FromStr;
use std::time::Duration;

use crate::config::DatabaseConfig;

// Initializes the database connection pool using the provided configuration.
pub async fn init(cfg: &DatabaseConfig) -> Result<SqlitePool, sqlx::Error> {
    let dsn = dsn(cfg);
    let opts = SqliteConnectOptions::from_str(&dsn)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .foreign_keys(true)
        .busy_timeout(Duration::from_secs(cfg.timeout_seconds));

    Ok(SqlitePoolOptions::new()
        .max_connections(cfg.max_connections)
        .acquire_timeout(Duration::from_secs(cfg.timeout_seconds))
        .connect_with(opts)
        .await?)
}

// Generates the Data Source Name (DSN) for the SQLite database based on the provided configuration.
fn dsn(cfg: &DatabaseConfig) -> String {
    format!("sqlite://{}?mode=rwc", cfg.path)
}
