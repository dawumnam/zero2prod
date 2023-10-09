use std::net::TcpListener;

use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let path = format!("http://{}/health-check", address.to_string());
    let client = reqwest::Client::new();
    let response = client
        .get(path)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind");
    let address= listener.local_addr().unwrap().to_string();
    let server = run(listener).unwrap();

    tokio::spawn(server);

    address
}
