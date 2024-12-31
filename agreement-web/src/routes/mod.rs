use axum::{
    routing::{get, post},
    Router,
};

pub mod index;
pub mod thing_create;

pub fn router() -> Router {
    Router::new()
        .route("/", get(index::route))
        .route("/things", post(thing_create::route))
}
