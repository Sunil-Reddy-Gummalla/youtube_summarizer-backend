use axum::{Router, routing::post};
use crate::handlers::summarize_handler::summarize;

pub fn summarize_routes() -> Router {
    Router::new()
        .route("/summarize", post(summarize))
}
