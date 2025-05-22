use axum::routing::post;
use axum::{Router, routing::get};
use axum_sqs_lib::my_extractors;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/foo", post(post_foo).get(my_extractors::headers))
        .route("/echo", post(my_extractors::echo_bytes))
        .route("/users/{user_id}", get(my_extractors::path_param))
        .route("/users", get(my_extractors::query))
        .route("/headers", get(my_extractors::headers))
        .route("/input-string", post(my_extractors::input_string))
        .route("/json", post(my_extractors::input_json))
        .route("/sample-request", post(my_extractors::sample_request))
        .route("/string-handler", get(my_extractors::string_handler));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// which calls one of these handlers
async fn post_foo() {}
// async fn get_foo() {}
// async fn foo_bar() {}
