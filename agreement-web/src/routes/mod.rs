use std::sync::Arc;

use axum::{Router, routing::get};

use crate::state::AState;

mod api;

mod page_index;
mod page_man;

pub fn router() -> Router<Arc<AState>> {
    Router::new()
        .route("/", get(page_index::route))
        .route("/man", get(page_man::route))
        .route("/api/v1/man", get(api::man_get::route))
}
