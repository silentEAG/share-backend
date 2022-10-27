use axum::{routing::get, Router};

mod create;

pub fn router() -> Router {
    Router::new().route("/create", get(create::handler))
}
