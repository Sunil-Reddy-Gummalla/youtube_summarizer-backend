use crate::handlers::summarize_handler::summarize;
use axum::{Router, routing::post};

pub fn summarize_routes() -> Router {
    Router::new().route("/summarize", post(summarize))
}
