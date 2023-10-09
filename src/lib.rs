use hyper::server::conn::AddrIncoming;
use std::net:: TcpListener;

use axum::{
    http::StatusCode,
    routing::{get, IntoMakeService},
    Router, Server,
};

pub fn run(listener:TcpListener) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, anyhow::Error> {
    let app = Router::new().route("/health-check", get(health_check));
    let server = axum::Server::from_tcp(listener).expect("failed to build from tcp").serve(app.into_make_service());

    Ok(server)
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
