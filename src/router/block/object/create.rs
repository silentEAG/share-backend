use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub async fn handler(Path(block_id): Path<String>) -> crate::Result<Response> {
    tracing::info!(block_id);
    Ok(Json(json!({
        "status": "ok"
    }))
    .into_response())
}
