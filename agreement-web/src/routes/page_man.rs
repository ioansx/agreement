use agreement_error::Errx;
use askama::Template;
use axum::{http::header, response::Response};

use crate::error::WebResultx;

#[derive(Template)]
#[template(path = "page_man.html")]
struct PageTemplate;

pub async fn route() -> WebResultx<Response<String>> {
    let value = PageTemplate
        .render()
        .map_err(|e| Errx::einternal(e, "SSR failure"))?;
    Response::builder()
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("text/html"),
        )
        .body(value)
        .map_err(|e| Errx::einternal(e, "error creating the response").into())
}
