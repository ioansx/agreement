use agreement_models::outdto::ManGetOutdto;
use axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;

use crate::{error::WebResultx, state::ArcState};

#[derive(Deserialize)]
pub struct ManGetQueryParams {
    pub page: String,
}

pub async fn route(
    State(state): State<ArcState>,
    Query(query): Query<ManGetQueryParams>,
) -> WebResultx<Json<ManGetOutdto>> {
    state.validation0.man.validate_man_get(&query.page)?;

    let outdto = state.services.man.generate_man_page(query.page).await?;

    Ok(Json(outdto))
}
