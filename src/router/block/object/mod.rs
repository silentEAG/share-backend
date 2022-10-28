use axum::{
    routing::{get, post},
    Router,
};

mod get;
mod put;

pub fn router() -> Router {
    Router::new()
        .route("/put", post(put::handler))
        .route("/get", get(get::handler))
}
