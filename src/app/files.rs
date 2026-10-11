// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

pub mod directory;
mod file;
mod folder;

use crate::service::{FileService, StorageService};

use axum::{
    Router,
    extract::FromRef,
    routing::{get, post},
};

#[derive(Clone, FromRef)]
pub struct FileRouterState {
    pub svc: FileService,
    pub storage_svc: StorageService,
}

// Constructs the file router with routes for directories, files, folders, and uploads.
pub fn router() -> Router<FileRouterState> {
    Router::new()
        .route("/", get(directory::root))
        .route(
            "/{key}",
            get(file::file)
                .post(file::update_file)
                .patch(file::update_file)
                .delete(file::delete_file),
        )
        .route("/{key}/download", get(file::download_file))
        .route("/{key}/move", get(file::move_file))
        .route("/upload", post(file::upload_file))
        .route(
            "/folder/{folder_id}",
            get(directory::directory)
                .post(folder::update_folder)
                .patch(folder::update_folder),
        )
        .route("/folder/{folder_id}/move", get(folder::move_folder))
        .route("/folder", post(folder::create_folder))
}
