//! Authentication Middleware Module
//! 
//! This module provides JWT-based authentication middleware for protected routes.
//! It handles token validation and user context management using task-local storage.

use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use axum::http::StatusCode;
use crate::auth_claim::{Keys, Claims};
use tokio::task_local;
use axum::extract::Request;
use axum::response::Response;
use axum::middleware::Next;
use axum::RequestExt;
use jsonwebtoken::{Validation, decode};
use std::sync::LazyLock;

/// JWT signing keys for token validation
/// 
/// Initialized from the `JWT_SECRET` environment variable
static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    let secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

/// Represents the current authenticated user
/// 
/// This struct holds the user information extracted from the JWT token
#[derive(Clone)]
#[allow(dead_code)]
struct CurrentUser {
    name: String,
}

task_local! {
    /// Task-local storage for the current user.
    /// This allows accessing the current user's information throughout the request lifecycle.
    pub static USER: CurrentUser;
}

/// Authentication middleware that validates JWT tokens
/// 
/// This middleware:
/// 1. Extracts the Bearer token from the Authorization header
/// 2. Validates the JWT token
/// 3. Creates a user context from the token claims
/// 4. Makes the user context available to subsequent handlers
/// 
/// # Arguments
/// 
/// * `req` - The incoming request
/// * `n` - The next middleware in the chain
/// 
/// # Returns
/// 
/// A `Result` containing either:
/// * `Ok(Response)` - The response from the next middleware
/// * `Err(StatusCode)` - `UNAUTHORIZED` if authentication fails
pub async fn auth(mut req: Request, n: Next) -> Result<Response, StatusCode> {
    // Extract the authorization header
    let auth_header = req
        .extract_parts::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Decode and validate the token
    let token_data = decode::<Claims>(
        auth_header.0.token(),
        &KEYS.decoding,
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Create current user from token claims
    let cur_usr = CurrentUser {
        name: token_data.claims.company,
    };

    println!("at auth: {}", cur_usr.name);

    // Run the next middleware with the current user in scope
    Ok(USER.scope(cur_usr, n.run(req)).await)
}