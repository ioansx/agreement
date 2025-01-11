use agreement_common::{error::Err, newer};
use askama::Template;
use axum::{http::header, response::Response};

use crate::error::{Aerr, AerrResult};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

pub async fn route() -> AerrResult<Response<String>> {
    let value = IndexTemplate
        .render()
        .map_err(|e| Aerr(newer!(e, Err::internal("SSR failure"))))?;
    Response::builder()
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static(IndexTemplate::MIME_TYPE),
        )
        .body(value)
        .map_err(|e| Aerr(newer!(e, Err::internal("error creating the response"))))
}
