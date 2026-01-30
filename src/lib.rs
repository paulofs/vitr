use axum::{Router, http::StatusCode, routing};

// Root handler.
async fn root() -> &'static str {
    "Hello, World!"
}

// Return `200 OK` if the server is running.
async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub fn app() -> Router {
    Router::new()
        .route("/", routing::get(root))
        .route("/health_check", routing::get(health_check))
}
