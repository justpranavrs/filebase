// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use askama::Template;
use axum::{
    Form,
    extract::{Path, Query, State},
    response::Html,
};
use serde::Deserialize;

use crate::{
    app::{
        Error,
        toast::{ToastTemplate, ToastType},
    },
    models::{Folder, FolderBreadcrumb, NewFolder, ROOT_FOLDER_ID, UpdateFolder},
    service::FileService,
};

// Query parameters for navigating destination folders in the move modal.
#[derive(Debug, Deserialize)]
pub struct TargetFolderQuery {
    pub target_folder_id: Option<i64>,
}

// Template definition for the move folder modal drill-down navigator.
#[derive(Template)]
#[template(path = "directory/move_folder.html")]
struct MoveFolderTemplate {
    folder: Folder,
    current_folder: Folder,
    ancestors: Vec<FolderBreadcrumb>,
    subfolders: Vec<Folder>,
}

impl MoveFolderTemplate {
    pub fn is_same_directory(&self) -> bool {
        let current_parent_id = self.folder.parent_id.unwrap_or(ROOT_FOLDER_ID);
        self.current_folder.id == current_parent_id
    }

    pub fn can_move(&self) -> bool {
        !self.is_same_directory()
            && self.current_folder.id != self.folder.id
            && !self.ancestors.iter().any(|a| a.id == self.folder.id)
    }
}

// Template definition for rendering an individual folder list item as HTML.
#[derive(Template)]
#[template(path = "directory/folder_item.html")]
pub struct FolderItemTemplate {
    pub folder: Folder,
}

// Conversion implementation to wrap a Folder model into its corresponding template.
impl From<Folder> for FolderItemTemplate {
    // Wraps a Folder model into a FolderItemTemplate.
    fn from(f: Folder) -> Self {
        Self { folder: f }
    }
}

// Request handler to create a new folder from form data and return its rendered HTML fragment with toast.
pub async fn create_folder(
    State(svc): State<FileService>,
    Form(folder): Form<NewFolder>,
) -> Result<Html<String>, Error> {
    let folder = svc.create_folder(folder).await?;
    let toast = ToastTemplate {
        toast_type: ToastType::Success,
        message: "folder created successfully".into(),
    }
    .render()?;

    Ok(Html(format!("{}{}", FolderItemTemplate { folder }, toast)))
}

// Request handler to update a folder's name or parent location and return the updated card with toast.
pub async fn update_folder(
    State(svc): State<FileService>,
    Path(folder_id): Path<i64>,
    Form(update): Form<UpdateFolder>,
) -> Result<Html<String>, Error> {
    let is_move = update.parent_id.is_some();
    let folder = svc.update_folder(folder_id, update).await?;

    let message = if is_move {
        "folder moved successfully"
    } else {
        "folder renamed successfully"
    };

    let toast = ToastTemplate {
        toast_type: ToastType::Success,
        message: message.into(),
    }
    .render()?;

    if is_move {
        Ok(Html(format!(
            r#"<div id="folder-{}" hx-swap-oob="delete"></div>{}"#,
            folder_id, toast
        )))
    } else {
        Ok(Html(format!("{}{}", FolderItemTemplate { folder }, toast)))
    }
}

// Request handler to render the move folder modal drill-down navigator.
pub async fn move_folder(
    State(svc): State<FileService>,
    Path(folder_id): Path<i64>,
    Query(query): Query<TargetFolderQuery>,
) -> Result<Html<String>, Error> {
    let folder = svc.folder_by_id(folder_id).await?;
    let target_id = query
        .target_folder_id
        .or(folder.parent_id)
        .unwrap_or(ROOT_FOLDER_ID);
    let (current_folder, ancestors, subfolders) = svc.move_navigator(target_id).await?;

    Ok(Html(
        MoveFolderTemplate {
            folder,
            current_folder,
            ancestors,
            subfolders,
        }
        .render()?,
    ))
}
