use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AState;

mod man_get;
mod page_index;
mod page_man;
mod thing_create;

pub fn router() -> Router<Arc<AState>> {
    Router::new()
        .route("/", get(page_index::route))
        .route("/man", get(page_man::route))
        .route("/api/v1/man", get(man_get::route))
        .route("/api/v1/things", post(thing_create::route))
}
