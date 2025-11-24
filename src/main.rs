mod routes;
mod handlers;
mod services;
mod models;
mod utils;


use axum::{Router, routing::get};
use dotenvy::dotenv;
use std::env;
use routes::summarize::summarize_routes;





#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));

    let app = Router::new().merge(summarize_routes()).route("/", get(|| async { "Hello, World!" }));

    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
