// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

// Constructs the static router with services for robots.txt, favicon.ico, and static assets.
pub fn router() -> Router {
    Router::new()
        .route_service("/robots.txt", ServeFile::new("static/robots.txt"))
        .route_service("/favicon.ico", ServeFile::new("static/favicon.svg"))
        .nest_service("/static", ServeDir::new("static"))
}
