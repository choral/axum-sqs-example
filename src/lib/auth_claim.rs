//! Authentication Claims Module
//! 
//! This module handles JWT token generation, validation, and claims processing.
//! It provides functionality for:
//! - User authentication and token generation
//! - JWT token validation
//! - Claims extraction from requests
//! - Error handling for authentication failures

use axum::{
    Json, RequestPartsExt,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use dotenvy;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;
use std::sync::LazyLock;

/// JWT signing keys for token encoding and decoding
/// 
/// Initialized from the `JWT_SECRET` environment variable
static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    let secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

/// JWT signing keys container
/// 
/// Holds both encoding and decoding keys for JWT operations
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    /// Creates new JWT signing keys from a secret
    /// 
    /// # Arguments
    /// 
    /// * `secret` - The secret key bytes used for JWT signing
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

/// Handles user authentication and token generation
/// 
/// Validates user credentials and generates a JWT token if valid.
/// 
/// # Arguments
/// 
/// * `Json(payload)` - The authentication payload containing client credentials
/// 
/// # Returns
/// 
/// A `Result` containing either:
/// * `Ok(Json<AuthBody>)` - The generated JWT token
/// * `Err(AuthError)` - If authentication fails
pub async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    // Here you can check the user credentials from a database
    if payload.client_id != "foo" || payload.client_secret != "bar" {
        return Err(AuthError::WrongCredentials);
    }
    let claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    println!("Client Authorised: {}", claims.company);
    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}

/// Authentication error types
#[derive(Debug)]
pub enum AuthError {
    /// Invalid client credentials provided
    WrongCredentials,
    /// Missing required credentials
    MissingCredentials,
    /// Failed to create JWT token
    TokenCreation,
    /// Invalid or malformed JWT token
    InvalidToken,
}

/// JWT claims structure
/// 
/// Contains the data that will be encoded in the JWT token
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (typically user identifier)
    pub sub: String,
    /// Company or organization identifier
    pub company: String,
    /// Token expiration timestamp
    pub exp: usize,
}

/// Authentication response body
/// 
/// Contains the generated JWT token and its type
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthBody {
    /// The JWT access token
    pub access_token: String,
    /// The type of token (always "Bearer")
    pub token_type: String,
}

/// Authentication request payload
/// 
/// Contains the credentials needed for authentication
#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    /// Client identifier
    pub client_id: String,
    /// Client secret
    pub client_secret: String,
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "sub:{} Company:{}, exp:{}",
            self.sub, self.company, self.exp
        )
    }
}

/// Implementation of `FromRequestParts` for `Claims`
/// 
/// Allows automatic extraction of `Claims` from request parts
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    /// Extracts and validates JWT claims from the request
    /// 
    /// # Arguments
    /// 
    /// * `parts` - The request parts containing the authorization header
    /// * `_state` - The application state (unused)
    /// 
    /// # Returns
    /// 
    /// A `Result` containing either:
    /// * `Ok(Claims)` - The validated claims from the JWT token
    /// * `Err(AuthError)` - If token extraction or validation fails
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

impl AuthBody {
    /// Creates a new authentication response body
    /// 
    /// # Arguments
    /// 
    /// * `access_token` - The JWT token to be returned
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

/// Implementation of `IntoResponse` for `AuthError`
/// 
/// Converts authentication errors into appropriate HTTP responses
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
