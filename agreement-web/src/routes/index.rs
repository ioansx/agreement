use agreement_models::{error::ErKind, newer};
use askama::Template;
use axum::{
    http::header,
    response::{IntoResponse, Response},
};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[axum::debug_handler]
pub async fn route() -> impl IntoResponse {
    let value = IndexTemplate
        .render()
        .map_err(|e| newer!(e, ErKind::internal("SSR failure")))?;
    Response::builder()
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static(IndexTemplate::MIME_TYPE),
        )
        .body(value)
        .map_err(|e| newer!(e, ErKind::internal("error creating the response")))
}
