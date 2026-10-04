// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

mod error;
mod files;
mod health;

pub use error::Error;

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
    config::ServerConfig,
    queries::{FileDB, FolderDB},
    service::FileService,
};

// Define a constant for the "x-request-id" header name
const X_REQUEST_ID: HeaderName = HeaderName::from_static("x-request-id");

// Starts the server with the given configuration and returns the bind address as a string.
pub async fn start_server(cfg: &ServerConfig, pool: SqlitePool) -> io::Result<String> {
    let app = create_router(cfg, pool);
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
fn create_router(cfg: &ServerConfig, pool: SqlitePool) -> Router {
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

    let file_service = FileService::new(file_db, folder_db);
    let file_router = files::new(file_service);

    let app = Router::new().nest("/folders", file_router).layer(layer);
    Router::new()
        .route("/health", get(health::health_handler))
        .nest(&cfg.base_path, app)
}
