// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

mod directory;
mod folder;

use crate::service::FileService;

use axum::{
    Router,
    routing::{get, post},
};

pub fn new(svc: FileService) -> Router {
    Router::new()
        .route("/", get(directory::root))
        .route("/", post(folder::create_folder))
        .route("/{folder_id}", get(directory::directory))
        .with_state(svc)
}
