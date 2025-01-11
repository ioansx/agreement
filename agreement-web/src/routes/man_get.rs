use agreement_models::{indto::ManGetIndto, outdto::ManGetOutdto};
use axum::{extract::State, Json};

use crate::{
    error::{AEr, AResult},
    state::ArcState,
};

pub async fn route(
    State(state): State<ArcState>,
    Json(indto): Json<ManGetIndto>,
) -> AResult<ManGetOutdto> {
    // TODO: validation
    state
        .services
        .man
        .generate_man_page(indto)
        .await
        .map_err(|e| AEr(e))
}
