use agreement_models::{indto::ManGetIndto, outdto::ManGetOutdto};
use axum::{extract::State, Json};

use crate::{
    error::{Aerr, Aresult},
    state::ArcState,
};

pub async fn route(
    State(state): State<ArcState>,
    Json(indto): Json<ManGetIndto>,
) -> Aresult<Json<ManGetOutdto>> {
    state
        .validators
        .man
        .sanity_check_man_get(&indto)
        .map_err(|e| Aerr(e))?;

    let outdto = state
        .services
        .man
        .generate_man_page(indto)
        .await
        .map_err(|e| Aerr(e))?;

    Ok(Json(outdto))
}
