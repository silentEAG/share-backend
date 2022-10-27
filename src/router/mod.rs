use axum::{routing::get, Router};

pub mod block;
pub mod pong;

pub async fn router() -> Router {
    Router::new()
        .route("/", get(pong::handler))
        .nest("/block", block::router())
}
