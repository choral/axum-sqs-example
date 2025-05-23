#[cfg(test)]
#[tokio::test]
async fn my_test() {
    println!("hello world");
}

#[test]
fn add_two_and_two() {
    use axum_sqs_lib::my_math;
    let result = my_math::add_two(2);
    assert_eq!(result, 4);
}

use axum_sqs_lib::{
    auth_claim::AuthBody,
    backend_server,
};
use reqwest::{Client, StatusCode};
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;

/// Helper function to start the test server
/// 
/// Returns a tuple containing:
/// - The server address
/// - An HTTP client
async fn spawn_test_server() -> (SocketAddr, Client) {
    // Start the server on a random port
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    // Get the router from backend_server
    let app = backend_server::init_app();
    
    // Spawn the server in a background task
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // Create a client
    let client = Client::new();

    (addr, client)
}

#[tokio::test]
async fn test_hello_world() {
    let (addr, client) = spawn_test_server().await;
    
    let response = client
        .get(format!("http://{}/", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.text().await.unwrap(), "Hello, World!");
}

#[tokio::test]
async fn test_authentication_flow() {
    let (addr, client) = spawn_test_server().await;

    // Test missing credentials
    let response = client
        .post(format!("http://{}/authorization", addr))
        .json(&json!({
            "client_id": "",
            "client_secret": ""
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Test wrong credentials
    let response = client
        .post(format!("http://{}/authorization", addr))
        .json(&json!({
            "client_id": "wrong",
            "client_secret": "wrong"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // Test successful authentication
    let response = client
        .post(format!("http://{}/authorization", addr))
        .json(&json!({
            "client_id": "foo",
            "client_secret": "bar"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let auth_body: AuthBody = response.json().await.unwrap();
    assert_eq!(auth_body.token_type, "Bearer");
    assert!(!auth_body.access_token.is_empty());

    // Test protected endpoint with valid token
    let response = client
        .post(format!("http://{}/protected", addr))
        .header("Authorization", format!("Bearer {}", auth_body.access_token))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_protected_endpoints() {
    let (addr, client) = spawn_test_server().await;

    // Get a valid token first
    let auth_response = client
        .post(format!("http://{}/authorization", addr))
        .json(&json!({
            "client_id": "foo",
            "client_secret": "bar"
        }))
        .send()
        .await
        .unwrap();

    let auth_body: AuthBody = auth_response.json().await.unwrap();
    let token = format!("Bearer {}", auth_body.access_token);

    // Test protected endpoint without token
    let response = client
        .post(format!("http://{}/protected", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // Test protected endpoint with token
    let response = client
        .post(format!("http://{}/protected", addr))
        .header("Authorization", &token)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let text = response.text().await.unwrap();
    assert!(text.contains("Welcome to the protected area"));
    assert!(text.contains("ACME")); // Company name from claims

    // Test protected endpoint with input
    let response = client
        .post(format!("http://{}/protected/norm", addr))
        .header("Authorization", &token)
        .body("test input")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.text().await.unwrap(), "test input");
}

#[tokio::test]
async fn test_input_handlers() {
    let (addr, client) = spawn_test_server().await;

    // Test string handler
    let response = client
        .get(format!("http://{}/string-handler", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.text().await.unwrap(), "Hello, from string handler!");

    // Test JSON input
    let user_detail = json!({
        "user_id": 1,
        "username": "Test User",
        "is_active": true
    });

    let response = client
        .post(format!("http://{}/json", addr))
        .json(&user_detail)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Test query parameters
    let response = client
        .get(format!("http://{}/users?page=1&per_page=10", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_error_handling() {
    let (addr, client) = spawn_test_server().await;

    // Test invalid JSON
    let response = client
        .post(format!("http://{}/json", addr))
        .body("invalid json")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);

    // Test invalid path parameter
    let response = client
        .get(format!("http://{}/users/invalid", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
