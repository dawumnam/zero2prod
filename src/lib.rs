use std::net::SocketAddr;

use axum::{http::StatusCode, routing::get, Router};

pub async fn run() {
    let app = Router::new().route("/health-check", get(health_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
