// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use askama::Template;
use axum::{
    http::{HeaderMap, StatusCode},
    response::Response,
};

use crate::app::{Error, render};

// Template definition for the application 404 fallback page.
#[derive(Template)]
#[template(path = "fallback.html")]
struct FallbackTemplate {}

// Request handler to handle fallback 404 pages.
pub async fn fallback(headers: HeaderMap) -> Result<(StatusCode, Response), Error> {
    let page = render(headers, FallbackTemplate {}.render()?)?;
    Ok((StatusCode::NOT_FOUND, page))
}
