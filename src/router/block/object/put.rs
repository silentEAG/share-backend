use axum::{
    extract::{ContentLengthLimit, Multipart, Path},
    response::{IntoResponse, Response},
    Extension, Json,
};
use entity::{block, object, prelude::Block};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, Set,
};
use serde_json::json;

use crate::{error::ServerError, S3};

const MAX_UPLOAD_SIZE: u64 = 1024 * 1024 * 512;

/// Put Object in a block
pub async fn handler(
    Path(block_uid): Path<String>,
    Extension(ref conn): Extension<DatabaseConnection>,
    ContentLengthLimit(mut multipart): ContentLengthLimit<Multipart, { MAX_UPLOAD_SIZE }>,
) -> crate::Result<Response> {
    let block = Block::find()
        .filter(block::Column::BlockUid.contains(&block_uid))
        .one(conn)
        .await?;

    if let Some(block) = block {
        if let Some(file) = multipart.next_field().await.unwrap() {
            let filename = file.file_name().unwrap().to_string();
            let content_type = file.content_type().unwrap().to_string();
            let data = Vec::from(file.bytes().await.unwrap());
            let uinque_obj_name = uuid::Uuid::new_v4();
            let data_len = data.len() as i64;
            let object_key = format!("{}/{}", block.block_bucket_path, uinque_obj_name);
            S3.put_object(
                content_type.clone(),
                data_len,
                object_key.clone(),
                data.into(),
            )
            .await?;

            let _ = object::ActiveModel {
                id: NotSet,
                object_name: Set(format!("{}/{}", block.id, filename.clone())),
                object_type: Set(content_type.clone()),
                object_size: Set(data_len),
                object_description: Set("".to_string()),
                object_bucket_name: Set(uinque_obj_name.to_string()),
                block_id: Set(block.id),
            }
            .save(conn)
            .await?;
            return Ok(Json(json!({
                "status": "ok",
                "object_name": filename,
                "object_size": data_len,
                "object_type": content_type
            }))
            .into_response());
        }
        Ok(Json(json!({
            "status": "ok"
        }))
        .into_response())
    } else {
        Err(ServerError::OtherWithMessage(
            "Can't find block.".to_string(),
        ))
    }
}
