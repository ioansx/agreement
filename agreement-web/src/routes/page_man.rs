use agreement_common::{error::Err, newer};
use askama::Template;
use axum::{http::header, response::Response};

use crate::error::{Aerr, AerrResult};

#[derive(Template)]
#[template(path = "page_man.html")]
struct PageTemplate;

pub async fn route() -> AerrResult<Response<String>> {
    let value = PageTemplate
        .render()
        .map_err(|e| Aerr(newer!(e, Err::internal("SSR failure"))))?;
    Response::builder()
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static(PageTemplate::MIME_TYPE),
        )
        .body(value)
        .map_err(|e| Aerr(newer!(e, Err::internal("error creating the response"))))
}
