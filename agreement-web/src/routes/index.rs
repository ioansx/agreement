use agreement_common::{error::Er, newer};
use askama::Template;
use axum::{
    http::header,
    response::{IntoResponse, Response},
};

use crate::error::AgEr;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[axum::debug_handler]
pub async fn route() -> impl IntoResponse {
    let value = IndexTemplate
        .render()
        .map_err(|e| AgEr(newer!(e, Er::internal("SSR failure"))))?;
    Response::builder()
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static(IndexTemplate::MIME_TYPE),
        )
        .body(value)
        .map_err(|e| AgEr(newer!(e, Er::internal("error creating the response"))))
}
