use agreement_common::agreement_id;
use agreement_models::{indto::CreateThingIndto, outdto::CreateThingOutdto};
use axum::{http::StatusCode, Json};

pub async fn route(Json(payload): Json<CreateThingIndto>) -> (StatusCode, Json<CreateThingOutdto>) {
    let thing = CreateThingOutdto {
        id: agreement_id(),
        name: payload.name,
        opt_prop: payload.opt_prop,
    };

    (StatusCode::CREATED, Json(thing))
}
