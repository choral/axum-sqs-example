//! Protected Router Module
//! 
//! This module provides routes that require authentication to access.
//! It includes middleware for JWT token validation and protected endpoints
//! that can only be accessed with valid authentication.

use crate::auth_claim::{AuthError, Claims};
use crate::auth_claim_mid::auth;
use axum::http::StatusCode;
use axum::middleware::{self};
use axum::{Router, routing::post};

/// Creates a new router with protected routes
/// 
/// The router includes:
/// - A root endpoint (`/`) that returns protected data
/// - A normalized endpoint (`/norm`) that processes input text
/// - Authentication middleware that validates JWT tokens
/// 
/// # Returns
/// 
/// A configured `Router` with protected routes and authentication middleware
pub fn router() -> Router {
    Router::new()
        .route("/", post(protected))
        .route("/norm", post(protected_norm))
        .layer(middleware::from_fn(auth))
}

/// Protected endpoint handler
/// 
/// This endpoint requires a valid JWT token and returns the claims data
/// from the token. It demonstrates how to access authenticated user data.
/// 
/// # Arguments
/// 
/// * `claims` - The JWT claims containing user information
/// 
/// # Returns
/// 
/// A `Result` containing either:
/// * `Ok(String)` - A welcome message with the user's claims data
/// * `Err(AuthError)` - If there's an authentication error
pub async fn protected(claims: Claims) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{claims}",
    ))
}

/// Protected endpoint with input text processing
/// 
/// This endpoint requires a valid JWT token and processes the provided
/// input text. It demonstrates how to handle both authentication and
/// request data in a protected endpoint.
/// 
/// # Arguments
/// 
/// * `claims` - The JWT claims containing user information
/// * `input_text` - The text to be processed
/// 
/// # Returns
/// 
/// A `Result` containing either:
/// * `Ok(String)` - The processed input text
/// * `Err(StatusCode)` - If there's an error processing the request
pub async fn protected_norm(claims: Claims, input_text: String) -> Result<String, StatusCode> {
    let text_data = input_text;
    println!("input lxt: {} \n claims: {}", text_data, claims);
    Ok(text_data)
}

