use async_openai::{Client, config::OpenAIConfig, types::CreateCompletionRequestArgs};
use axum::Json;
use dotenvy::dotenv;
use std::env;
use axum::http::StatusCode;
use crate::models::summarize_response::SummarizeResponse;

pub async fn summarize_text(text: &str) -> (StatusCode, Json<SummarizeResponse>) {
    dotenv().ok();

    if let Ok(api_key) = env::var("OPENROUTER_API_KEY") {
        let config = OpenAIConfig::new()
            .with_api_base("https://openrouter.ai/api/v1")
            .with_api_key(api_key);
        let client = Client::with_config(config);
        if let Ok(request) = CreateCompletionRequestArgs::default()
            .model("x-ai/grok-4.1-fast:free")
            .prompt(format!("Summerize this Youtube Transcript: /n {text}"))
            .build()
        {
            if let Ok(response) = client.completions().create(request).await {
                let summary = response.choices[0].text.clone();
                return (StatusCode::OK, Json(SummarizeResponse { summary: Some(summary), error: None}));
            } else {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(SummarizeResponse { summary: None, error: Some("Error from response of llm".to_string())}));
            }
        } else {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(SummarizeResponse { summary: None, error: Some("Error creating request".to_string())}));
        }
    } else {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(SummarizeResponse { summary: None, error: Some("Error getting api key".to_string())}));
    }
}
