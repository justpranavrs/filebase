// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use super::Error;

use askama::Template;
use axum::{http::HeaderMap, response::Html};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    content: String,
}

pub fn render(headers: HeaderMap, fragment: String) -> Result<Html<String>, Error> {
    let htmx =
        headers.contains_key("hx-request") && !headers.contains_key("hx-history-restore-request");
    if htmx {
        Ok(Html(fragment))
    } else {
        Ok(Html(IndexTemplate { content: fragment }.render()?))
    }
}

pub async fn index() -> Result<Html<String>, Error> {
    let fragment = r#"
        <div class="folder-item">
            <a
                hx-get='/folders/1'
                hx-target='#app'
                hx-swap='innerHTML'
                hx-push-url='true'
            >
                Enter Filebase
            </a>
        </div>
    "#;
    Ok(Html(
        IndexTemplate {
            content: fragment.into(),
        }
        .render()?,
    ))
}
