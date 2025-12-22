use crate::models::summarize_response::SummarizeResponse;
use async_openai::{Client, config::OpenAIConfig, types::CreateCompletionRequestArgs};
use axum::Json;
use axum::http::StatusCode;
use dotenvy::dotenv;
use std::env;

pub async fn summarize_text(text: &str) -> (StatusCode, Json<SummarizeResponse>) {
    dotenv().ok();

    if let Ok(api_key) = env::var("OPENROUTER_API_KEY") {
        let config = OpenAIConfig::new()
            .with_api_base("https://openrouter.ai/api/v1")
            .with_api_key(api_key);
        let client = Client::with_config(config);
        if let Ok(request) = CreateCompletionRequestArgs::default()
            .model("nex-agi/deepseek-v3.1-nex-n1:free")
            .prompt(format!(
                "Summerize this Youtube Transcript in english: /n {text}"
            ))
            .build()
        {
            match client.completions().create(request).await {
                Ok(response) => {
                    let summary = response.choices[0].text.clone();
                    (
                        StatusCode::OK,
                        Json(SummarizeResponse {
                            summary: Some(summary),
                            error: None,
                        }),
                    )
                }
                Err(e) => {
                    eprintln!("Error from LLM API: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(SummarizeResponse {
                            summary: None,
                            error: Some("Error from response of llm".to_string()),
                        }),
                    )
                }
            }
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SummarizeResponse {
                    summary: None,
                    error: Some("Error creating request".to_string()),
                }),
            )
        }
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SummarizeResponse {
                summary: None,
                error: Some("Error getting api key".to_string()),
            }),
        )
    }
}
