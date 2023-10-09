use zero2prod::run;

#[tokio::main]
async fn main() {
    run()
        .expect("Failed to bind address")
        .await
        .expect("Failed to run server")
}
