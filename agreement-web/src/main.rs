use std::net::{Ipv4Addr, SocketAddr};

use agreement_web::{
    error::{ErKind, Eresult},
    newer, routes,
};
use axum::{
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::{
    services::ServeDir,
    trace::{DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Eresult<()> {
    tracing_subscriber::fmt::init();

    let middleware = ServiceBuilder::new().layer(
        TraceLayer::new_for_http()
            .on_response(DefaultOnResponse::new().latency_unit(LatencyUnit::Micros)),
    );

    let app = Router::new()
        .route("/", get(routes::index::route))
        .route("/users", post(routes::user_create::route))
        .nest_service("/static", ServeDir::new("static"))
        .layer(middleware);

    let port = std::env::var("PORT").unwrap_or_else(|_| "80".to_string());
    let port: u16 = port.parse().unwrap_or_else(|e| {
        error!("Invalid port number {}; parsing failed: {}", port, e);
        std::process::exit(1);
    });
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| newer!(e, ErKind::internal("unable to bind address")))?;

    info!("Listening at {}", addr);
    if let Err(e) = axum::serve(listener, app).await {
        error!("{}", e);
    }

    Ok(())
}
