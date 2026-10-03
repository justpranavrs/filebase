// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use axum::Json;
use serde::Serialize;

// Defines the structure for the health check response, containing a message and status.
#[derive(Serialize)]
pub struct Health {
    message: &'static str,
    status: &'static str,
}

// Handles the health check endpoint, returning a JSON response with a welcome message and status.
pub async fn health_handler() -> Json<Health> {
    Json(Health { message: "welcome to filebase!", status: "ok" })
}