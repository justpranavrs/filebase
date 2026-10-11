// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

mod app;
mod config;
mod db;
mod logs;
mod models;
mod queries;
mod service;
mod storage;

use tracing::info;

// Application entry point that initializes configuration, logging, storage, database, and server.
#[tokio::main]
async fn main() {
    let cfg = config::load().expect("invalid configuration loaded");
    println!("filebase configuration loaded");

    let log_level = logs::init(&cfg.log);
    info!("logging initialized with level: {}", log_level);

    storage::init(&cfg.storage).expect("could not initialize file storage");

    let pool = db::init(&cfg.db)
        .await
        .expect("could not initialize database connection pool");
    info!("database connection pool initialized");

    let bind_address = app::start_server(&cfg.server, &cfg.storage, pool.clone())
        .await
        .expect("failed to initialize server");
    info!("server shutdown on {}", bind_address);

    pool.close().await;
    info!("shutdown complete");
}
