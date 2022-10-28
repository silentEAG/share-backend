use std::collections::HashMap;

use crate::S3;
use axum::{
    body::StreamBody,
    extract::{Path, Query},
    http::{header, StatusCode},
    response::{AppendHeaders, IntoResponse},
    Extension,
};
use entity::{
    block, object,
    prelude::{Block, Object},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tokio_util::io::ReaderStream;

/// Get Object in a block
pub async fn handler(
    Path(block_uid): Path<String>,
    Extension(ref conn): Extension<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let obj_uid = params.get("obj_uid").unwrap();
    println!("{}", obj_uid);

    let block = Block::find()
        .filter(block::Column::BlockUid.eq(block_uid.clone()))
        .one(conn)
        .await
        .unwrap()
        .unwrap();

    let object = Object::find()
        .filter(object::Column::BlockId.eq(block.id))
        .filter(object::Column::ObjectBucketName.eq(obj_uid.to_string()))
        .one(conn)
        .await
        .unwrap()
        .unwrap();

    let obj_key = format!("{}/{}", block.block_bucket_path, object.object_bucket_name);
    let data = S3.get_object(obj_key).await.unwrap();

    let _ = match tokio::fs::File::open("Cargo.toml").await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };

    let stream = ReaderStream::new(data.into_async_read());

    let body = StreamBody::new(stream);

    let headers = AppendHeaders([
        (
            header::CONTENT_TYPE,
            format!("{}; charset=utf-8", object.object_type),
        ),
        (
            header::CONTENT_DISPOSITION,
            format!(
                "attachment; filename=\"{}\"",
                object.object_name.split_once('/').unwrap().1
            ),
        ),
    ]);
    Ok((headers, body))
}
