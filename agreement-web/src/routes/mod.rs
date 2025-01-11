use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::state::ArState;

pub mod index_get;
pub mod man_get;
pub mod thing_create;

pub fn router() -> Router<Arc<ArState>> {
    Router::new()
        .route("/", get(index_get::route))
        .route("/man", get(man_get::route))
        .route("/things", post(thing_create::route))
}
