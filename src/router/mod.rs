use axum::{
    routing::{get, post},
    Router,
};

pub mod auth;
pub mod block;
pub mod pong;

pub async fn router() -> Router {
    Router::new()
        .route("/", get(pong::handler))
        .route("/auth", post(auth::handler))
        .nest("/block", block::router())
}
