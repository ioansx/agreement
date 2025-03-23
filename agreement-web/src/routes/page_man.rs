use agreement_error::{Errx, Resultx};
use askama::Template;
use axum::{http::header, response::Response};

#[derive(Template)]
#[template(path = "page_man.html")]
struct PageTemplate;

pub async fn route() -> Resultx<Response<String>> {
    let value = PageTemplate
        .render()
        .map_err(|e| Errx::einternal(e, "SSR failure"))?;
    Response::builder()
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static(PageTemplate::MIME_TYPE),
        )
        .body(value)
        .map_err(|e| Errx::einternal(e, "error creating the response"))
}
