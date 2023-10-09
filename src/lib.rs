use hyper::server::conn::AddrIncoming;
use std::net::TcpListener;

use axum::{
    http::StatusCode,
    routing::{get, post, IntoMakeService},
    Form, Json, Router, Server,
};

#[derive(serde::Deserialize)]
struct SubscriberForm {
    email: String,
    name: String,
}

pub fn run(
    listener: TcpListener,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, anyhow::Error> {
    let app = Router::new()
        .route("/health-check", get(health_check))
        .route("/subscriptions", post(subscribe));
    let server = axum::Server::from_tcp(listener)
        .expect("failed to build from tcp")
        .serve(app.into_make_service());

    Ok(server)
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn subscribe(Form(payload): Form<SubscriberForm>) -> StatusCode {
    StatusCode::OK
}
