// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use askama::Template;
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::Html,
};

use crate::{
    app::{Error, render},
    models::{Folder, ROOT_FOLDER_ID},
    service::{Directory, FileService, FolderItems},
};

#[derive(Template)]
#[template(path = "directory/index.html")]
struct DirectoryTemplate {
    parent: Folder,
    items: FolderItems,
}

impl From<Directory> for DirectoryTemplate {
    fn from(d: Directory) -> Self {
        Self {
            parent: d.parent,
            items: d.items,
        }
    }
}

pub async fn root(
    State(svc): State<FileService>,
    headers: HeaderMap,
) -> Result<Html<String>, Error> {
    let fragment = DirectoryTemplate::from(svc.folder_items(ROOT_FOLDER_ID).await?).render()?;
    render(headers, fragment)
}

pub async fn directory(
    State(svc): State<FileService>,
    Path(folder_id): Path<i64>,
    headers: HeaderMap,
) -> Result<Html<String>, Error> {
    let fragment = DirectoryTemplate::from(svc.folder_items(folder_id).await?).render()?;
    render(headers, fragment)
}
