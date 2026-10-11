// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use std::io;

#[derive(Debug)]
pub enum Error {
    Database(sqlx::Error),
    NotFound,
    Storage(io::Error),
    InvalidOperation(String),
}

// Conversion implementation from database layer errors into service errors.
impl From<sqlx::Error> for Error {
    // Converts a sqlx database error into a service error.
    fn from(e: sqlx::Error) -> Self {
        Self::Database(e)
    }
}

// Conversion implementation from disk layer errors into service errors.
impl From<io::Error> for Error {
    // Converts an I/O storage error into a service error.
    fn from(e: io::Error) -> Self {
        Self::Storage(e)
    }
}
