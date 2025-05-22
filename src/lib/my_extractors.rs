use crate::input_schemas::{GetUserWithId, Pagination, UserDetail};
use axum::{
    body::Bytes,
    extract::{Json, Path, Query, Request},
    http::{
        StatusCode,
        header::{self, HeaderMap},
    },
};

// `Path` gives you the path parameters and deserializes them.
// for this example, /users/<> is the path
pub async fn path_param(Path(GetUserWithId { user_id }): Path<GetUserWithId>) {
    println!("user_id={}", user_id);
}

// `Query` gives you the query parameters and deserializes them.
// this sample parse query string '?page=<>per_page=<>' into pagination
pub async fn query(Query(Pagination { page, per_page }): Query<Pagination>) {
    println!("user_id: {}    name:{}", page, per_page);
}

// `HeaderMap` gives you all the headers
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

pub async fn string_handler() -> String {
    "Hello, from string handler!".to_string()
}

// `Bytes` gives you the raw request body
pub async fn echo_bytes(body: Bytes) -> Result<String, StatusCode> {
    if let Ok(string) = String::from_utf8(body.to_vec()) {
        println!("String value: {}", string);
        Ok(string)
    } else {
        println!("body cant be converted to string!");
        Err(StatusCode::BAD_REQUEST)
    }
}

// `String` consumes the request body and ensures it is valid utf-8
pub async fn input_string(body: String) -> String {
    let mut output_string = String::from("Receive Body:");
    output_string.push_str(body.as_str());
    output_string
}

// We've already seen `Json` for parsing the request body as json
pub async fn input_json(Json(payload): Json<UserDetail>) {
    println!("{:?}", payload);
}

// `Request` gives you the whole request for maximum control
pub async fn sample_request(req: Request) {
    let method = req.method();
    let uri = req.uri();
    println!("method : {},   uri: {}", method, uri);
}

// `Extension` extracts data from "request extensions"
// This is commonly used to share state with handlers
// async fn extension(Extension(state): Extension<State>) {}
