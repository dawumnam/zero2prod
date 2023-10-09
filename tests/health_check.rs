use std::{net::TcpListener, thread::spawn};

use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health-check", spawn_app()))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let client = reqwest::Client::new();
    let body = "name=dawum%20nam&email=dawumnam%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", spawn_app()))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_form_data() {
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=dawumnam", "missing the email"),
        ("email=dawumnam%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
        ("name=nam&email=not-an-email", "the email is not an email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", spawn_app()))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind");
    let address = listener.local_addr().unwrap().to_string();
    let server = run(listener).unwrap();

    tokio::spawn(server);

    format!("http://{}", address)
}
