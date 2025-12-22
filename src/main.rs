mod handlers;
mod models;
mod routes;
mod services;
mod utils;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use routes::summarize::summarize_routes;
use std::env;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .merge(summarize_routes())
        .route("/", get(|| async { "Hello, World!" }))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
