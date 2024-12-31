use axum::{routing::get, Router};

pub mod index;
pub mod user_create;

pub fn router() -> Router {
    Router::new().route("/", get(index::route))
}
