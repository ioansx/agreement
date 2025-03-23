use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};

use agreement_error::Errx;
use agreement_web::{routes, state::AState};
use axum::Router;
use tower::ServiceBuilder;
use tower_http::{
    LatencyUnit,
    services::{ServeDir, ServeFile},
    trace::{DefaultOnResponse, TraceLayer},
};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let middleware = ServiceBuilder::new().layer(
        TraceLayer::new_for_http()
            .on_response(DefaultOnResponse::new().latency_unit(LatencyUnit::Micros)),
    );

    let state = AState::new();

    let app = Router::new()
        .merge(routes::router())
        .nest_service(
            "/favicon.ico",
            ServeFile::new(PathBuf::from("static/favicon.ico")),
        )
        .nest_service("/static", ServeDir::new("static"))
        .layer(middleware)
        .with_state(Arc::new(state));

    let port = std::env::var("PORT").unwrap_or_else(|_| "80".to_string());
    let port: u16 = port.parse().unwrap_or_else(|e| {
        error!("Invalid port number {}; parsing failed: {}", port, e);
        std::process::exit(1);
    });
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| Errx::einternal(e, "Unable to bind address."))?;

    info!("Listening at {}", addr);
    if let Err(e) = axum::serve(listener, app).await {
        error!("{}", e);
    }

    Ok(())
}
