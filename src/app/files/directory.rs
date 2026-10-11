// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use askama::Template;
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::Response,
};

use crate::{
    app::{
        Error,
        files::{file::FileItemTemplate, folder::FolderItemTemplate},
        render,
    },
    models::{Folder, FolderBreadcrumb, ROOT_FOLDER_ID},
    service::{Directory, FileService},
};

// Template definition for rendering the main directory view containing the parent folder and its items.
#[derive(Template)]
#[template(path = "directory/index.html")]
struct DirectoryTemplate {
    parent: Folder,
    ancestors: Vec<FolderBreadcrumb>,
    folders: Vec<FolderItemTemplate>,
    files: Vec<FileItemTemplate>,
}

// Conversion implementation to map a Directory domain model directly into the DirectoryTemplate.
impl From<Directory> for DirectoryTemplate {
    // Converts a Directory domain model into a DirectoryTemplate.
    fn from(d: Directory) -> Self {
        Self {
            parent: d.parent,
            ancestors: d.ancestors,
            folders: d
                .folders
                .into_iter()
                .map(FolderItemTemplate::from)
                .collect(),
            files: d.files.into_iter().map(FileItemTemplate::from).collect(),
        }
    }
}

// Request handler to render the root directory view using the default root folder ID.
pub async fn root(State(svc): State<FileService>, headers: HeaderMap) -> Result<Response, Error> {
    let fragment = DirectoryTemplate::from(svc.folder_items(ROOT_FOLDER_ID).await?).render()?;
    render(headers, fragment)
}

// Request handler to render a specific directory view based on the folder ID path parameter.
pub async fn directory(
    State(svc): State<FileService>,
    Path(folder_id): Path<i64>,
    headers: HeaderMap,
) -> Result<Response, Error> {
    let fragment = DirectoryTemplate::from(svc.folder_items(folder_id).await?).render()?;
    render(headers, fragment)
}
