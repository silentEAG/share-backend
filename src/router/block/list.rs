use std::collections::HashMap;

use axum::{
    extract::Query,
    response::{IntoResponse, Response},
    Extension, Json,
};
use entity::prelude::Block;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde_json::json;

pub async fn handler(
    Extension(ref conn): Extension<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> crate::Result<Response> {
    let blocks = Block::find()
        // .order_by_asc(block::Column::Id)
        .all(conn)
        .await?;
    tracing::debug!("{:?}", params);
    Ok(Json(json!({
        "status": "ok",
        "block_cnt": blocks.len(),
        "blocks": blocks
    }))
    .into_response())
}
