use axum::{
    response::{IntoResponse, Response},
    Extension, Json,
};
use entity::{block, object, prelude::Block};
use rayon::prelude::*;
use rusoto_s3::ObjectIdentifier;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, TransactionTrait,
};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct BlockDelete {
    block_uid: String,
}

use crate::{error::ServerError, S3, router::auth::Claims};
pub async fn handler(
    Json(req): Json<BlockDelete>,
    Extension(ref conn): Extension<DatabaseConnection>,
    _: Claims
) -> crate::Result<Response> {
    let txn = conn.begin().await?;

    let block = Block::find()
        .filter(block::Column::BlockUid.eq(req.block_uid))
        .one(&txn)
        .await?;

    if let Some(block) = block {
        let objects = object::Entity::find()
            .filter(object::Column::BlockId.eq(block.id))
            .all(conn)
            .await?;

        let vec_object: Vec<ObjectIdentifier> = objects
            .into_par_iter()
            .map(|object| {
                let key = format!("{}/{}", block.block_bucket_path, object.object_bucket_name);
                ObjectIdentifier {
                    key,
                    version_id: None,
                }
            })
            .collect();

        if !vec_object.is_empty() {
            S3.delete_objects(vec_object).await?;
        }

        let result = object::Entity::delete_many()
            .filter(object::Column::BlockId.eq(block.id))
            .exec(&txn)
            .await?;

        block.delete(&txn).await?;

        txn.commit().await?;
        Ok(Json(json!({
            "status": "ok",
            "object_num": result.rows_affected
        }))
        .into_response())
    } else {
        txn.commit().await?;
        Err(ServerError::OtherWithMessage(
            "Block is not found.".to_string(),
        ))
    }
}
