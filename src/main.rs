//! Axum web application with authentication and request tracing
//! 
//! This is the binary entry point for the application.
//! It handles environment configuration and starts the web server.

use axum_sqs_lib::backend_server::run_server;
use dotenvy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Start the server
    run_server().await
}

