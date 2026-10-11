// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use axum::{
    extract::multipart,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use crate::app::toast::{ToastTemplate, ToastType};

// Application-level error enum wrapping service layer and template rendering errors.
pub enum Error {
    Multipart(multipart::MultipartError),
    Render(askama::Error),
    Service(crate::service::Error),
    Status(StatusCode),
}

// Conversion implementation from multipart errors into application errors.
impl From<multipart::MultipartError> for Error {
    // Converts a multipart extraction error into an application error.
    fn from(e: multipart::MultipartError) -> Self {
        Self::Multipart(e)
    }
}

// Conversion implementation from Askama template errors into application errors.
impl From<askama::Error> for Error {
    // Converts an Askama template rendering error into an application error.
    fn from(e: askama::Error) -> Self {
        Self::Render(e)
    }
}

// Conversion implementation from service layer errors into application errors.
impl From<crate::service::Error> for Error {
    // Converts a service layer error into an application error.
    fn from(e: crate::service::Error) -> Self {
        Self::Service(e)
    }
}

// Conversion implementation from status codes into application errors.
impl From<StatusCode> for Error {
    // Converts an HTTP status code into an application error.
    fn from(s: StatusCode) -> Self {
        Self::Status(s)
    }
}

// Implementation of Axum's IntoResponse for the application Error enum to map errors.
impl IntoResponse for Error {
    // Maps an application error to an HTTP status code and an HTML error toast response.
    fn into_response(self) -> Response {
        use crate::service::Error as E;

        let (status, message) = match &self {
            Self::Multipart(e) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid file upload or form data: {}", e),
            ),
            Self::Render(e) => {
                tracing::error!("failed to render HTML template: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to load the page layout. Please try again".into(),
                )
            }
            Self::Service(E::Database(e)) => {
                tracing::error!("database error occurred: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal database error occurred. Please try again later.".into(),
                )
            }
            Self::Service(E::NotFound) => (
                StatusCode::NOT_FOUND,
                "The requested resource could not be found".into(),
            ),
            Self::Service(E::InvalidOperation(message)) => {
                (StatusCode::BAD_REQUEST, message.clone())
            }
            Self::Service(E::Storage(e)) => {
                tracing::error!("storage error occurred: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal storage error occurred. Please try again later.".into(),
                )
            }
            Self::Status(StatusCode::BAD_REQUEST) => (
                StatusCode::BAD_REQUEST,
                "Invalid or malformed request body".into(),
            ),
            Self::Status(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong".into(),
            ),
        };
        (
            status,
            Html(ToastTemplate {
                toast_type: ToastType::Error,
                message,
            }),
        )
            .into_response()
    }
}
