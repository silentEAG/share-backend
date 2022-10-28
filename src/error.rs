#![allow(dead_code)]
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Db(#[from] sea_orm::error::DbErr),
    #[error("Error with message: `{0}`")]
    OtherWithMessage(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ServerError::IO(error) => (StatusCode::BAD_REQUEST, error.to_string()),
            ServerError::Db(error) => (StatusCode::BAD_REQUEST, error.to_string()),
            ServerError::OtherWithMessage(m) => (StatusCode::BAD_REQUEST, m),
            _ => (
                StatusCode::BAD_REQUEST,
                "Oops! Unknown Error :(".to_string(),
            ),
        };
        let body = Json(json!({
            "status": "error",
            "message": error_message,
        }));
        (status, body).into_response()
    }
}
