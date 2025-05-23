use crate::my_extractors;
use axum::{Router, routing::get};
// pub fn api_router() -> Router {
//     Router::new()
//         .nest("/users", user::router())
// }

pub fn router() -> Router {
    Router::new()
        .route("/", get(my_extractors::query))
        .route("/{user_id}", get(my_extractors::path_param))
}
