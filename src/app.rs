// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

mod error;
mod fallback;
mod files;
mod health;
mod index;
mod toast;

pub use error::Error;
pub use index::render;

use std::{io, time::Duration};

use axum::{
    Router,
    http::{HeaderName, StatusCode},
    routing::get,
    serve,
};
use sqlx::SqlitePool;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::{
    LatencyUnit,
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::{DefaultOnResponse, TraceLayer},
};
use tracing::{Level, info};

use crate::{
    config::{ServerConfig, StorageConfig},
    queries::{FileDB, FolderDB},
    service::{FileService, StorageService},
};

// Define a constant for the "x-request-id" header name
const X_REQUEST_ID: HeaderName = HeaderName::from_static("x-request-id");

// Starts the server with the given configuration and returns the bind address as a string.
pub async fn start_server(
    cfg: &ServerConfig,
    strg_cfg: &StorageConfig,
    pool: SqlitePool,
) -> io::Result<String> {
    let app = create_router(cfg, strg_cfg, pool);
    let bind_address = format!("{}:{}", cfg.host, cfg.port);
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;

    info!("server listening on {}", bind_address);
    serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(bind_address)
}

// Creates a shutdown signal that triggers on ctrl+c or SIGTERM.
pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install ctrl+c handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("received ctrl+c, gracefully shutting down")
        },
        _ = terminate => {
            info!("received SIGTERM, gracefully shutting down")
        },
    }
}

// Creates a router with the specified server configuration, setting up various middleware layers for request handling.
fn create_router(cfg: &ServerConfig, strg_cfg: &StorageConfig, pool: SqlitePool) -> Router {
    let layer = ServiceBuilder::new()
        .layer(SetRequestIdLayer::new(X_REQUEST_ID, MakeRequestUuid))
        .layer(
            TraceLayer::new_for_http().on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
                    .latency_unit(LatencyUnit::Millis),
            ),
        )
        .layer(CatchPanicLayer::new())
        .layer(CompressionLayer::new().gzip(true))
        .layer(PropagateRequestIdLayer::new(X_REQUEST_ID))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(cfg.timeout_seconds),
        ));

    let file_db = FileDB::new(pool.clone());
    let folder_db = FolderDB::new(pool.clone());

    let storage_service = StorageService::new(strg_cfg.clone());

    let file_service = FileService::new(file_db, folder_db);
    let state = files::FileRouterState {
        svc: file_service,
        storage_svc: storage_service,
    };

    Router::new()
        .route("/", get(files::directory::root))
        .route("/health", get(health::health_handler))
        .nest("/files", files::router())
        .fallback(fallback::fallback)
        .with_state(state)
        .layer(layer)
}
