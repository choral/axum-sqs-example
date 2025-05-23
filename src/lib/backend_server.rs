//! Backend server module
//! 
//! This module provides the core functionality for setting up and running the web server,
//! including route configuration, middleware setup, and server initialization.

use axum::{Router, routing::get, extract::Extension, routing::post};
use crate::{app_state, auth_claim, my_extractors, protected_router, users_router};
use tower_http::trace::{TraceLayer, DefaultMakeSpan, DefaultOnResponse};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Initialize the application router with all routes and middleware
/// 
/// This function sets up the Axum router with all routes, middleware,
/// and application state. It can be used both for the main server
/// and for testing.
pub fn init_app() -> Router {
    // Initialize application state
    let shared_app_state = app_state::MyAppState {
        db_enpoint: String::from("this is db enpoint string"),
        is_connected: false,
        conntection_string: String::from("this is connection string"),
    };

    // Build the application router with all routes and middleware
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/users", users_router::router())
        .nest("/protected", protected_router::router())
        .route("/foo", post(post_foo).get(my_extractors::headers))
        .route("/echo", post(my_extractors::echo_bytes))
        .route("/headers", get(my_extractors::headers))
        .route("/input-string", post(my_extractors::input_string))
        .route("/json", post(my_extractors::input_json))
        .route("/sample-request", post(my_extractors::sample_request))
        .route("/string-handler", get(my_extractors::string_handler))
        .route("/authorization", post(auth_claim::authorize))
        .layer(Extension(shared_app_state))
        // Add request tracing middleware
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
}

/// Start the server with configuration from environment variables
/// 
/// The server configuration is read from environment variables:
/// - `HOST`: The host address to bind to
/// - `PORT`: The port number to listen on
/// - `RUST_LOG`: The logging level (defaults to "info")
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing with configurable log level
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Configure server address from environment variables
    let mut host_variable = dotenvy::var("HOST").unwrap();
    let port = dotenvy::var("PORT").unwrap();
    host_variable.push_str(String::from(":").as_str());
    host_variable.push_str(port.as_str());
    println!("HOST:Port={}", host_variable);

    // Get the router
    let app = init_app();

    // Start the server
    let listener = tokio::net::TcpListener::bind(host_variable).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Handler for POST requests to /foo
/// 
/// This is a placeholder handler that currently does nothing.
/// It can be extended to handle POST requests to the /foo endpoint.
async fn post_foo() {}
