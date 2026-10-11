// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use askama::Template;
use axum::{
    extract::{Form, Multipart, Path, Query, State},
    http::{
        HeaderMap, HeaderValue, StatusCode,
        header::{CACHE_CONTROL, CONTENT_DISPOSITION, CONTENT_TYPE, ETAG, IF_NONE_MATCH},
    },
    response::{Html, IntoResponse, Response},
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    app::{
        Error,
        toast::{ToastTemplate, ToastType},
    },
    models::{File, Folder, FolderBreadcrumb, NewFile, UpdateFile},
    service::{FileService, StorageService},
    storage::checksum,
};

// Query parameters for navigating destination folders in the move modal.
#[derive(Debug, Deserialize)]
pub struct TargetFolderQuery {
    pub target_folder_id: Option<i64>,
}

// Template definition for the move file modal drill-down navigator.
#[derive(Template)]
#[template(path = "directory/move_file.html")]
struct MoveFileTemplate {
    file: File,
    current_folder: Folder,
    ancestors: Vec<FolderBreadcrumb>,
    subfolders: Vec<Folder>,
}

impl MoveFileTemplate {
    pub fn is_same_directory(&self) -> bool {
        self.file.folder_id == self.current_folder.id
    }

    pub fn can_move(&self) -> bool {
        !self.is_same_directory()
    }
}

enum Disposition {
    Attachment,
    Inline,
}

// Template definition for rendering an individual file list item as HTML.
#[derive(Template)]
#[template(path = "directory/file_item.html")]
pub struct FileItemTemplate {
    pub file: File,
}

impl From<File> for FileItemTemplate {
    // Wraps a File model into a FileItemTemplate.
    fn from(f: File) -> Self {
        Self { file: f }
    }
}

// Formats the Content-Disposition header string for file downloads.
fn resolve_content_disposition(disposition: Disposition, name: &str) -> String {
    match disposition {
        Disposition::Attachment => format!("attachment; filename=\"{}\"", name),
        Disposition::Inline => format!("inline; filename=\"{}\"", name),
    }
}

// Builds response headers including Content-Type, Content-Disposition, and ETag checksum.
fn resolve_headers(file: &File, disposition: Disposition) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    if let Ok(val) = file.content_type.parse() {
        headers.insert(CONTENT_TYPE, val);
    }
    if let Ok(val) = resolve_content_disposition(disposition, &file.name).parse() {
        headers.insert(CONTENT_DISPOSITION, val);
    }
    if let Ok(val) = format!("\"{}\"", file.checksum).parse() {
        headers.insert(ETAG, val);
    }
    headers
}

// Checks if the incoming request's If-None-Match header matches the file checksum.
fn is_not_modified(headers: &HeaderMap, checksum: &str) -> bool {
    let Some(if_none_match) = headers.get(IF_NONE_MATCH) else {
        return false;
    };
    let Ok(header_str) = if_none_match.to_str() else {
        return false;
    };
    header_str.split(',').any(|tag| {
        let tag = tag.trim();
        tag == "*" || tag.trim_matches('"') == checksum
    })
}

// Generates a 304 Not Modified response with ETag and Cache-Control headers.
fn not_modified_response(checksum: &str) -> Response {
    let mut headers = HeaderMap::new();
    headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    if let Ok(val) = format!("\"{}\"", checksum).parse() {
        headers.insert(ETAG, val);
    }
    (StatusCode::NOT_MODIFIED, headers).into_response()
}

// Request handler to view a file inline by serving bytes directly from storage.
pub async fn file(
    State(svc): State<FileService>,
    State(storage_svc): State<StorageService>,
    Path(key): Path<Uuid>,
    headers: HeaderMap,
) -> Result<Response, Error> {
    let file = svc.file_by_key(key).await?;
    if is_not_modified(&headers, &file.checksum) {
        return Ok(not_modified_response(&file.checksum));
    }
    let data = storage_svc.read_file(&file.object_key).await?;
    let resp_headers = resolve_headers(&file, Disposition::Inline);
    Ok((resp_headers, data).into_response())
}

// Request handler to download a file attachment by serving bytes directly from storage.
pub async fn download_file(
    State(svc): State<FileService>,
    State(storage_svc): State<StorageService>,
    Path(key): Path<Uuid>,
    headers: HeaderMap,
) -> Result<Response, Error> {
    let file = svc.file_by_key(key).await?;
    if is_not_modified(&headers, &file.checksum) {
        return Ok(not_modified_response(&file.checksum));
    }
    let data = storage_svc.read_file(&file.object_key).await?;
    let resp_headers = resolve_headers(&file, Disposition::Attachment);
    Ok((resp_headers, data).into_response())
}

// Request handler to process multipart file uploads and return the new file card with toast.
pub async fn upload_file(
    State(svc): State<FileService>,
    State(storage_svc): State<StorageService>,
    mut multipart: Multipart,
) -> Result<Html<String>, Error> {
    let mut folder_id: Option<i64> = None;
    let mut file: Option<File> = None;

    while let Some(field) = multipart.next_field().await? {
        let name = match field.name() {
            Some(name) => name.to_string(),
            None => continue,
        };

        if name == "folder_id" {
            let text_value = field.text().await?;
            folder_id = Some(
                text_value
                    .parse::<i64>()
                    .map_err(|_| Error::Status(StatusCode::BAD_REQUEST))?,
            );
            continue;
        }

        if let Some(file_name) = field.file_name() {
            let target_folder_id = folder_id.ok_or(Error::Status(StatusCode::BAD_REQUEST))?;
            let file_name = file_name.to_string();
            let content_type = field.content_type().map(ToString::to_string);
            let data = field.bytes().await?;
            let checksum = checksum(&data);
            let object_key = storage_svc.save_file(&data).await?;

            let new_file = NewFile {
                folder_id: target_folder_id,
                object_key,
                name: file_name,
                size: data.len() as i64,
                content_type,
                checksum,
            };
            file = Some(svc.create_file(new_file).await?);
        };
    }

    let file = file.ok_or(Error::Status(StatusCode::BAD_REQUEST))?;
    Ok(Html(format!(
        "{}{}",
        FileItemTemplate { file },
        ToastTemplate {
            toast_type: ToastType::Success,
            message: "file uploaded successfully".into(),
        }
        .render()?
    )))
}

// Request handler to update a file's name or folder and return the updated card with toast.
pub async fn update_file(
    State(svc): State<FileService>,
    Path(key): Path<Uuid>,
    Form(update): Form<UpdateFile>,
) -> Result<Html<String>, Error> {
    let is_move = update.folder_id.is_some();
    let file = svc.update_file(key, update).await?;

    let message = if is_move {
        "file moved successfully"
    } else {
        "file renamed successfully"
    };

    let toast = ToastTemplate {
        toast_type: ToastType::Success,
        message: message.into(),
    }
    .render()?;

    if is_move {
        Ok(Html(format!(
            r#"<div id="file-{}" hx-swap-oob="delete"></div>{}"#,
            key, toast
        )))
    } else {
        Ok(Html(format!("{}{}", FileItemTemplate { file }, toast)))
    }
}

// Request handler to render the move file modal drill-down navigator.
pub async fn move_file(
    State(svc): State<FileService>,
    Path(key): Path<Uuid>,
    Query(query): Query<TargetFolderQuery>,
) -> Result<Html<String>, Error> {
    let file = svc.file_by_key(key).await?;
    let target_id = query.target_folder_id.unwrap_or(file.folder_id);
    let (current_folder, ancestors, subfolders) = svc.move_navigator(target_id).await?;

    Ok(Html(
        MoveFileTemplate {
            file,
            current_folder,
            ancestors,
            subfolders,
        }
        .render()?,
    ))
}

// Request handler to delete a file from both the database and disk storage.
pub async fn delete_file(
    State(svc): State<FileService>,
    State(storage_svc): State<StorageService>,
    Path(key): Path<Uuid>,
) -> Result<Html<String>, Error> {
    svc.delete_file(key).await?;
    storage_svc.delete_file(&key).await?;

    let toast = ToastTemplate {
        toast_type: ToastType::Success,
        message: "file deleted successfully".into(),
    }
    .render()?;

    Ok(Html(format!(
        r#"<div id="file-{}" hx-swap-oob="delete"></div>{}"#,
        key, toast
    )))
}
