#[cfg(test)]
use once_cell::sync::Lazy;

use std::net::TcpListener;

use blog::{startup, telemetry};

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = telemetry::get_subscriber("test".into(), "debug".into());
    telemetry::init_subscriber(subscriber);
});

#[derive(serde::Serialize)]
struct TestUser {
    username: String,
    about: Option<String>,
    email: String,
    password: String,
}

fn spawn_app() -> String {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    let server = startup::run(listener).expect("Failed to spawn application.");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn signup_returns_201_for_valid_input_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let data = TestUser {
        username: "username".to_string(),
        about: None,
        email: "example@mail.com".to_string(),
        password: "password".to_string(),
    };

    let response = client
        .post(&format!("{}/users", address))
        .json(&data)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(201, response.status().as_u16());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn signup_returns_201_for_duplicate_username() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let data = TestUser {
        username: "NewUsername".to_string(),
        about: None,
        email: "newExample@mail.com".to_string(),
        password: "password".to_string(),
    };

    let response = client
        .post(&format!("{}/users", address))
        .json(&data)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(201, response.status().as_u16());

    let response = client
        .post(&format!("{}/users", address))
        .json(&data)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(409, response.status().as_u16());
}
