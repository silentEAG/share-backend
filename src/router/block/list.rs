use std::collections::HashMap;

use axum::extract::Query;

pub async fn handler(Query(params): Query<HashMap<String, String>>) -> crate::Result<String> {
    println!("{:?}", params);
    Ok("Yes".into())
}
