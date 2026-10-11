// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use askama::Template;
use axum::{
    http::{HeaderMap, HeaderValue, header::CACHE_CONTROL},
    response::{Html, IntoResponse, Response},
};

use crate::app::Error;

// Template definition for the main application layout wrapper containing dynamic content.
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    content: String,
}

// Helper function to render either just the HTMX HTML fragment or the full page layout.
pub fn render(headers: HeaderMap, fragment: String) -> Result<Response, Error> {
    let htmx = headers.contains_key("hx-request");
    let html = if htmx {
        Html(fragment)
    } else {
        Html(IndexTemplate { content: fragment }.render()?)
    };

    let mut response = html.into_response();
    response
        .headers_mut()
        .insert(CACHE_CONTROL, HeaderValue::from_static("no-store"));
    Ok(response)
}
