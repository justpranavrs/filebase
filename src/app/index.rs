// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use crate::app::Error;

use askama::Template;
use axum::{http::HeaderMap, response::Html};

// Template definition for the main application layout wrapper containing dynamic content.
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    content: String,
}

// Helper function to render either just the HTMX HTML fragment or the full page layout.
pub fn render(headers: HeaderMap, fragment: String) -> Result<Html<String>, Error> {
    let htmx =
        headers.contains_key("hx-request") && !headers.contains_key("hx-history-restore-request");
    if htmx {
        Ok(Html(fragment))
    } else {
        Ok(Html(IndexTemplate { content: fragment }.render()?))
    }
}
