// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("not found")]
    NotFound,

    #[error("already exists")]
    Conflict,

    #[error("forbidden")]
    Forbidden,

    #[error("database error")]
    Database(#[from] sqlx::Error),
}