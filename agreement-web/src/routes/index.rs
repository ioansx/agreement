use askama::Template;
use askama_axum::IntoResponse;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

pub async fn route() -> impl IntoResponse {
    IndexTemplate
}
