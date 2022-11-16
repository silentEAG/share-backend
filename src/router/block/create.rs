#![allow(dead_code)]
use axum::{
    response::{IntoResponse, Response},
    Extension, Json,
};
use entity::block;
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, DatabaseConnection, Set, TryIntoModel};
use serde::Deserialize;
use serde_json::json;

use crate::router::auth::Claims;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum BlockType {
    Send,
    Receive,
}

impl ToString for BlockType {
    fn to_string(&self) -> String {
        match self {
            BlockType::Send => String::from("send"),
            BlockType::Receive => String::from("receive"),
        }
    }
}

impl From<BlockType> for entity::block::BlockType {
    fn from(bt: BlockType) -> Self {
        match bt {
            BlockType::Send => entity::block::BlockType::Send,
            BlockType::Receive => entity::block::BlockType::Receive,
        }
    }
}

#[derive(Deserialize)]
pub struct BlockCreate {
    name: String,
    description: String,
    block_type: BlockType,
    block_format: Option<String>,
    block_fields: Option<Vec<String>>,
}

pub fn verify_data() -> bool {
    true
}

pub async fn handler(
    Json(req): Json<BlockCreate>,
    Extension(ref conn): Extension<DatabaseConnection>,
    _: Claims,
) -> crate::Result<Response> {
    let uuid = uuid::Uuid::new_v4();

    // TODO: verify data

    let block_path = format!("{}/{uuid}", req.block_type.to_string());

    let block = block::ActiveModel {
        id: NotSet,
        block_name: Set(req.name.clone()),
        block_uid: Set(uuid.to_string()),
        block_description: Set(req.description.clone()),
        block_type: Set(entity::block::BlockType::from(req.block_type)),
        block_bucket_path: Set(block_path.clone()),
        block_format: Set("".to_string()),
        block_fields: Set(vec![]),
        create_at: Set(chrono::Local::now().date_naive()),
    }
    .save(conn)
    .await?;

    let block = block.try_into_model()?;

    Ok(Json(json!({
        "status": "ok",
        "block": block
    }))
    .into_response())
}
