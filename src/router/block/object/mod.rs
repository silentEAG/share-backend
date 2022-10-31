use axum::{
    routing::{get, post},
    Router,
};

mod delete;
mod get;
mod list;
mod put;

pub fn router() -> Router {
    Router::new()
        .route("/put", post(put::handler))
        .route("/get", get(get::handler))
        .route("/list", get(list::handler))
        .route("/delete", post(delete::handler))
}
