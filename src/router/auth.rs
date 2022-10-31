use axum::Json;
use serde::Deserialize;

use crate::CONFIG;

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    secret_key: String,
}

pub async fn handler(Json(auth): Json<AuthPayload>) -> crate::Result<String> {
    if auth.secret_key == CONFIG.app_secret_key() {}
    Ok("Hello, SilentE!".into())
}
