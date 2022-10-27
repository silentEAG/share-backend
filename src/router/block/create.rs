use axum::{
    response::{IntoResponse, Response},
    Extension, Json,
};
use entity::block;
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, DatabaseConnection, Set};
use serde::Deserialize;
use serde_json::json;

use crate::S3;

#[derive(Deserialize)]
pub struct BlockCreate {
    name: String,
    description: String,
    block_type: String,
    block_format: Option<String>,
    block_fields: Option<Vec<String>>,
}

pub fn verify_data() -> bool {
    true
}

pub async fn handler(
    Json(req): Json<BlockCreate>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> crate::Result<Response> {
    let uuid = uuid::Uuid::new_v4();

    // TODO: verify data

    let block_path = format!("{}/{}", req.block_type, uuid);

    S3.put_object(0, format!("{}/index", block_path), Vec::new())
        .await?;

    let _ = block::ActiveModel {
        id: NotSet,
        block_name: Set(req.name),
        block_uid: Set(uuid.to_string()),
        block_description: Set(req.description),
        block_type: Set(req.block_type),
        block_bucket_path: Set(block_path),
        block_format: Set("".to_string()),
        block_fields: Set("".to_string()),
    }
    .save(conn)
    .await?;
    Ok(Json(json!({
        "status": "ok"
    }))
    .into_response())
}
