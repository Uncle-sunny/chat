mod sse;

use sse::*;

use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};

const CONTENT: &str = include_str!("../index.html");
pub fn get_router() -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/events", get(sse_handler))
}

async fn index_handler() -> impl IntoResponse {
    Html(CONTENT)
}
