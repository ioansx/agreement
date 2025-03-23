use agreement_error::Resultx;
use agreement_models::outdto::ManGetOutdto;
use axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;

use crate::state::ArcState;

#[derive(Deserialize)]
pub struct ManGetQueryParams {
    pub command: String,
}

pub async fn route(
    State(state): State<ArcState>,
    Query(query): Query<ManGetQueryParams>,
) -> Resultx<Json<ManGetOutdto>> {
    state.validation0.man.validate_man_get(&query.command)?;

    let outdto = state.services.man.generate_man_page(query.command).await?;

    Ok(Json(outdto))
}
