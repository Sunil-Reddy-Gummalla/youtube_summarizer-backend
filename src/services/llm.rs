use async_openai::{Client, config::OpenAIConfig, types::CreateCompletionRequestArgs};
use dotenvy::dotenv;
use std::env;

pub async fn summarize_text(text: &str) -> String {
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
                response.choices[0].text.clone()
            } else {
                return "Error from response of llm".to_string();
            }
        } else {
            return "Error creating request".to_string();
        }
    } else {
        return "API Key not found".to_string();
    }
}
