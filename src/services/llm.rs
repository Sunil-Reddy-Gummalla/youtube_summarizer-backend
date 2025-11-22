use async_openai::{
    Client,
    config::OpenAIConfig,
    types::{CreateCompletionRequestArgs},
};
use dotenvy::dotenv;
use std::env;

pub async fn summarize_text(text: &str) -> String {
    dotenv().ok();

    let api_key = env::var("OPENROUTER_API_KEY").unwrap();

    let config = OpenAIConfig::new()
        .with_api_base("https://openrouter.ai/api/v1")
        .with_api_key(api_key);

    let client = Client::with_config(config);

    let request = CreateCompletionRequestArgs::default()
        .model("x-ai/grok-4.1-fast:free")
        .prompt(format!("Summerize this Youtube Transcript: /n {text}"))
        .build().unwrap();
    let response = client.completions().create(request).await.unwrap();
    response.choices[0].text.clone()
}
