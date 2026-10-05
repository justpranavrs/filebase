// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use super::Error;

use askama::Template;
use axum::{
    Router, extract::{Path, State}, response::{Html, Result}, routing::get,
};

use crate::service::{FileService, FolderItems};

pub fn new(svc: FileService) -> Router {
    Router::new()
        .route("/{folder_id}", get(folder_items))
        .with_state(svc)
}

#[derive(Template)]
#[template(path = "folder.html")]
struct FolderTemplate {
    items: FolderItems,
}

pub async fn folder_items(State(svc): State<FileService>, Path(folder_id): Path<i64>) -> Result<Html<String>, Error> {
    let items = svc.folder_items(folder_id).await?;
    let folder = FolderTemplate{
        items,
    };
    Ok(Html(folder.render()?))
}
