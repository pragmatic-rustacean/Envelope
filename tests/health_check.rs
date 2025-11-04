#![allow(unused_variables)]

use std::net::TcpListener;

#[tokio::test]
async fn check_health_check() {
    // Arrange
    let url = spawn_app();
    // Use reqwest crate to perform HTTP requests against our application.

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", url))
        .send()
        .await
        .expect("Failed to execute request");

    // Asserts
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background.
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Retrieve the port assigned to use by the OS.
    let port = listener.local_addr().unwrap().port();
    let server = newslatter::run(listener).expect("Failed to bind port");
    let _ = tokio::spawn(server);
    // Return the application address to the caller!.
    format!("http://127.0.0.1:{}", port)
}
