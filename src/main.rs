// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

mod app;
mod config;
mod db;
mod logs;
mod models;
mod queries;
mod service;

use clap::Parser;
use tracing::info;

#[derive(Parser)]
#[command(name = "filebase", version = "0.1.0")]
struct Cli {
    #[arg(long)]
    migrate_only: bool,
}

// The main entrypoint for the application which initializes configuration, logger, database and server.
#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let cfg = config::load().expect("invalid configuration loaded");
    println!("filebase configuration loaded");

    let log_level = logs::init(&cfg.log);
    info!("logging initialized with level: {}", log_level);

    let pool = db::init(&cfg.db)
        .await
        .expect("could not initialize database connection pool");
    info!("database connection pool initialized");

    // Do the database migrations only if "migrate-only" flag is true.
    if args.migrate_only {
        info!("initiating database migrations");
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("failed to run database migrations");
        info!("database migrations completed");
        return;
    }

    let bind_address = app::start_server(&cfg.server, pool.clone())
        .await
        .expect("failed to initialize server");
    info!("server shutdown on {}", bind_address);

    pool.close().await;
    info!("shutdown complete");
}
