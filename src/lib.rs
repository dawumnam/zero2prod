use hyper::server::conn::AddrIncoming;
use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    routing::{get, IntoMakeService},
    Router, Server,
};

pub fn run() -> Result<Server<AddrIncoming, IntoMakeService<Router>>, anyhow::Error> {
    let app = Router::new().route("/health-check", get(health_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    Ok(server)
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
