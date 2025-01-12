use agreement_models::outdto::ManGetOutdto;
use axum::{
    extract::{Query, State},
    Json,
};

use crate::{
    error::{Aerr, AerrResult},
    state::ArcState,
};

pub async fn route(
    State(state): State<ArcState>,
    Query(command): Query<String>,
) -> AerrResult<Json<ManGetOutdto>> {
    state
        .validators
        .man
        .sanity_check_man_get(&command)
        .map_err(|e| Aerr(e))?;

    let outdto = state
        .services
        .man
        .generate_man_page(command)
        .await
        .map_err(|e| Aerr(e))?;

    Ok(Json(outdto))
}
