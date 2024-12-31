use agreement::{
    error::{ErKind, Eresult},
    newer, routes,
};
use axum::{
    routing::{get, post},
    Router,
};
use tracing::error;

#[tokio::main]
async fn main() -> Eresult<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(routes::root::route))
        .route("/users", post(routes::user_create::route));

    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| newer!(e, ErKind::internal("unable to bind address")))?;

    if let Err(e) = axum::serve(listener, app).await {
        error!("{}", e);
    }

    Ok(())
}
