use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let port = "3000";
    let path = format!("http://localhost:{}/health-check", port);
    let client = reqwest::Client::new();
    let response = client
        .get(path)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = run().unwrap();

    tokio::spawn(server);
}
