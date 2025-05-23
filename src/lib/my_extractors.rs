//! Request Extractors Module
//! 
//! This module demonstrates various ways to extract and handle different types of request data
//! in Axum, including:
//! - Path parameters
//! - Query parameters
//! - Headers
//! - Request body (raw bytes, string, JSON)
//! - Request extensions
//! - Full request handling

use crate::{
    app_state::MyAppState,
    input_schemas::{GetUserWithId, Pagination, UserDetail},
};
use axum::{
    Extension,
    body::Bytes,
    extract::{Json, Path, Query, Request},
    http::{
        StatusCode,
        header::{self, HeaderMap},
    },
};

/// Extracts and handles path parameters from the URL
/// 
/// # Arguments
/// 
/// * `Path(GetUserWithId { user_id })` - The user ID extracted from the URL path
/// 
/// # Example
/// 
/// For a request to `/users/123`, `user_id` will be `123`
pub async fn path_param(Path(GetUserWithId { user_id }): Path<GetUserWithId>) {
    println!("user_id={}", user_id);
}

/// Extracts and handles query parameters from the URL
/// 
/// # Arguments
/// 
/// * `Query(Pagination { page, per_page })` - The pagination parameters from the query string
/// 
/// # Example
/// 
/// For a request to `/users?page=1&per_page=10`, `page` will be `1` and `per_page` will be `10`
pub async fn query(Query(Pagination { page, per_page }): Query<Pagination>) {
    println!("user_id: {}    name:{}", page, per_page);
}

/// Extracts and handles HTTP headers from the request
/// 
/// # Arguments
/// 
/// * `headers` - The complete header map from the request
/// 
/// Currently handles:
/// - User-Agent header
/// - Content-Type header
pub async fn headers(headers: HeaderMap) {
    let user_agent = headers.get(header::USER_AGENT);
    let content_type = headers.get(header::CONTENT_TYPE);
    match user_agent {
        Some(usr_agent) => {
            println!("user_agent: {:?}", usr_agent)
        }
        None => todo!(),
    }
    match content_type {
        Some(c_type) => {
            println!("content type: {:?}", c_type);
        }
        None => {
            todo!()
        }
    }
}

/// Returns a simple string response
/// 
/// # Returns
/// 
/// A static string message
pub async fn string_handler() -> String {
    "Hello, from string handler!".to_string()
}

/// Handles raw request body as bytes
/// 
/// # Arguments
/// 
/// * `body` - The raw request body as bytes
/// 
/// # Returns
/// 
/// A `Result` containing either:
/// * `Ok(String)` - The body converted to a string
/// * `Err(StatusCode)` - If the body cannot be converted to UTF-8
pub async fn echo_bytes(body: Bytes) -> Result<String, StatusCode> {
    if let Ok(string) = String::from_utf8(body.to_vec()) {
        println!("String value: {}", string);
        Ok(string)
    } else {
        println!("body cant be converted to string!");
        Err(StatusCode::BAD_REQUEST)
    }
}

/// Handles request body as a string
/// 
/// # Arguments
/// 
/// * `body` - The request body as a string
/// 
/// # Returns
/// 
/// The input string prefixed with "Receive Body:"
pub async fn input_string(body: String) -> String {
    let mut output_string = String::from("Receive Body:");
    output_string.push_str(body.as_str());
    output_string
}

/// Handles JSON request body
/// 
/// # Arguments
/// 
/// * `Json(payload)` - The request body deserialized into a `UserDetail` struct
pub async fn input_json(Json(payload): Json<UserDetail>) {
    println!("{:?}", payload);
}

/// Demonstrates handling the full request and application state
/// 
/// # Arguments
/// 
/// * `Extension(state)` - The application state
/// * `req` - The complete request object
pub async fn sample_request(Extension(state): Extension<MyAppState>, req: Request) {
    let method = req.method();
    let uri = req.uri();
    println!("method : {},   uri: {}", method, uri);
    println!("state: {:?}", state);
}

// `Extension` extracts data from "request extensions"
// This is commonly used to share state with handlers
// async fn extension(Extension(state): Extension<State>) {}
