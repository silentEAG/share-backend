use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Extension, Json,
};
use entity::{
    block, object,
    prelude::{Block, Object},
};
use rayon::prelude::*;
use rusoto_s3::ObjectIdentifier;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, TransactionTrait,
};
use serde::Deserialize;
use serde_json::json;

use crate::{common::UnwrapOrError, error::ServerError, S3, router::auth::Claims};

#[derive(Deserialize)]
pub struct ObjectDelete {
    object_uids: Vec<String>,
}

pub async fn handler(
    Json(objects): Json<ObjectDelete>,
    Path(block_uid): Path<String>,
    Extension(ref conn): Extension<DatabaseConnection>,
    _: Claims
) -> crate::Result<Response> {
    let txn = conn.begin().await?;

    // Find block
    let block = Block::find()
        .filter(block::Column::BlockUid.eq(block_uid))
        .one(conn)
        .await?
        .unwrap_or_error(ServerError::OtherWithMessage(
            "Can't find Block.".to_string(),
        ))?;

    // Verify all object in this block
    // TODO: Change into parallelism
    for object_uid in &objects.object_uids {
        Object::find()
            .filter(object::Column::BlockId.eq(block.id))
            .filter(object::Column::ObjectBucketName.eq(object_uid.clone()))
            .one(conn)
            .await?
            .unwrap_or_error(ServerError::OtherWithMessage(
                "Can't find Object.".to_string(),
            ))?
            .delete(&txn)
            .await?;
    }

    // Use rayon for transforming into `ObjectIdentifier`
    let object_vec: Vec<ObjectIdentifier> = objects
        .object_uids
        .into_par_iter()
        .map(|uid| {
            let key = format!("{}/{uid}", block.block_bucket_path);
            ObjectIdentifier {
                key,
                version_id: None,
            }
        })
        .collect();
    let object_num = object_vec.len();

    S3.delete_objects(object_vec).await?;

    txn.commit().await?;

    Ok(Json(json!({
        "status": "ok",
        "object_num": object_num
    }))
    .into_response())
}
