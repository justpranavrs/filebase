// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use core::fmt;

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

pub enum ToastType {
    Success,
    Error,
}

impl fmt::Display for ToastType {
    // Formats the ToastType as its corresponding CSS class name.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Success => "toast-success",
            Self::Error => "toast-error",
        };
        write!(f, "{}", s)
    }
}

// Template definition for rendering error or notification toast messages via Askama.
#[derive(Template)]
#[template(path = "toast.html")]
pub struct ToastTemplate {
    pub toast_type: ToastType,
    pub message: String,
}

// Implementation of Axum's IntoResponse for ToastTemplate to render HTML or fall back to an internal server error.
impl IntoResponse for ToastTemplate {
    // Converts the ToastTemplate into an HTTP response by rendering its HTML.
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => {
                tracing::error!(error = %err, "failed to render template");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
