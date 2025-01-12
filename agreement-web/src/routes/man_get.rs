use agreement_models::outdto::ManGetOutdto;
use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::{
    error::{Aerr, AerrResult},
    state::ArcState,
};

#[derive(Deserialize)]
pub struct ManGetQueryParams {
    pub command: String,
}

pub async fn route(
    State(state): State<ArcState>,
    Query(query): Query<ManGetQueryParams>,
) -> AerrResult<Json<ManGetOutdto>> {
    state
        .validation0
        .man
        .validate_man_get(&query.command)
        .map_err(|e| Aerr(e))?;

    let outdto = state
        .services
        .man
        .generate_man_page(query.command)
        .await
        .map_err(|e| Aerr(e))?;

    Ok(Json(outdto))
}
