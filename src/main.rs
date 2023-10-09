use std::net::{SocketAddr, TcpListener};

use zero2prod::run;

#[tokio::main]
async fn main() {
    let address = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = TcpListener::bind(address).expect("Failed to bind");
    run(listener)
        .expect("Failed to bind address")
        .await
        .expect("Failed to run server")
}
