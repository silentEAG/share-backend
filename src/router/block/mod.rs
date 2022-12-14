use axum::{
    routing::{get, post},
    Router,
};

mod create;
mod delete;
mod info;
mod list;
mod object;

pub fn router() -> Router {
    Router::new()
        .route("/create", post(create::handler))
        .route("/list", get(list::handler))
        .route("/delete", post(delete::handler))
        .nest("/:block_uid", object::router())
}
