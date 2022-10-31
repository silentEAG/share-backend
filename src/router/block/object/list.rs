use std::collections::HashMap;

use crate::{common::UnwrapOrError, error::ServerError, S3};
use axum::{
    body::StreamBody,
    extract::{Path, Query},
    http::header,
    response::{AppendHeaders, IntoResponse, Response},
    Extension, Json,
};
use entity::{
    block, object,
    prelude::{Block, Object},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde_json::json;

/// List all object in a block
pub async fn handler(
    Path(block_uid): Path<String>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> crate::Result<Response> {
    let block = Block::find()
        .filter(block::Column::BlockUid.eq(block_uid.clone()))
        .one(conn)
        .await?
        .unwrap_or_error(ServerError::OtherWithMessage(
            "Can't find Block.".to_string(),
        ))?;

    let object = Object::find()
        .filter(object::Column::BlockId.eq(block.id))
        .all(conn)
        .await?;

    Ok(Json(json!({
        "status": "ok",
        "object_num": object.len(),
        "object": object
    }))
    .into_response())
}
