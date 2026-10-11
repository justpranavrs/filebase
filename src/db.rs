// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
};
use std::time::Duration;
use std::{fs, str::FromStr};
use tracing::info;

use crate::config::DatabaseConfig;

// Initializes the database connection pool using the provided configuration.
pub async fn init(cfg: &DatabaseConfig) -> Result<SqlitePool, sqlx::Error> {
    let opts = SqliteConnectOptions::from_str(&cfg.url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .foreign_keys(true)
        .busy_timeout(Duration::from_secs(cfg.timeout_seconds));

    // Create the database directory if it does not exist.
    if let Some(dir) = opts.get_filename().parent() {
        fs::create_dir_all(dir)?;
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(cfg.max_connections)
        .acquire_timeout(Duration::from_secs(cfg.timeout_seconds))
        .connect_with(opts)
        .await?;

    // Do the database migrations
    info!("initiating database migrations");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed to run database migrations");
    info!("database migrations completed");
    Ok(pool)
}
