use agreement_common::{error::Er, newer};
use askama::Template;
use axum::{http::header, response::Response};

use crate::error::{AEr, AResult};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

pub async fn route() -> AResult<Response<String>> {
    let value = IndexTemplate
        .render()
        .map_err(|e| AEr(newer!(e, Er::internal("SSR failure"))))?;
    Response::builder()
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static(IndexTemplate::MIME_TYPE),
        )
        .body(value)
        .map_err(|e| AEr(newer!(e, Er::internal("error creating the response"))))
}
