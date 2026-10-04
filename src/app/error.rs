// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use axum::response::{IntoResponse, Response};

pub enum Error {
    Service(crate::service::Error),
    Render(askama::Error),
}

impl From<crate::service::Error> for Error {
    fn from(e: crate::service::Error) -> Self { Self::Service(e) }
}

impl From<askama::Error> for Error {
    fn from(e: askama::Error) -> Self { Self::Render(e) }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        use crate::service::Error as E;
        use axum::http::StatusCode;

        let (status, msg) = match &self {
            Self::Service(E::NotFound) => (StatusCode::NOT_FOUND, "Not found"),
            Self::Service(E::Conflict) => (StatusCode::CONFLICT, "Already exists"),
            Self::Service(E::Forbidden) => (StatusCode::FORBIDDEN, "Not allowed"),
            Self::Service(_) | Self::Render(_) => {
                tracing::error!("request failed");
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
            }
        };
        (status, msg).into_response()
    }
}