// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use askama::Template;
use axum::{Form, extract::State, http::HeaderMap, response::Html};

use crate::{
    app::{Error, render},
    models::{Folder, NewFolder},
    service::FileService,
};

#[derive(Template)]
#[template(path = "directory/folder_item.html")]
struct FolderItemTemplate {
    folder: Folder,
}

impl From<Folder> for FolderItemTemplate {
    fn from(f: Folder) -> Self {
        Self { folder: f }
    }
}

#[axum::debug_handler]
pub async fn create_folder(
    State(svc): State<FileService>,
    headers: HeaderMap,
    Form(folder): Form<NewFolder>,
) -> Result<Html<String>, Error> {
    let fragment = FolderItemTemplate::from(svc.create_folder(folder).await?).render()?;
    render(headers, fragment)
}
