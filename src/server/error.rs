use std::fmt;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type ServerResult<T> = std::result::Result<T, ServerError>;

#[derive(Debug)]
pub enum ServerError {
    Postgres(tokio_postgres::Error),
}

impl From<tokio_postgres::Error> for ServerError {
    fn from(value: tokio_postgres::Error) -> Self {
        Self::Postgres(value)
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let body = match self {
            Self::Postgres(value) => value.to_string(),
        };

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Postgres(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for ServerError {}
