use crate::{
    models::summarize_request::SummarizeRequest, services::llm::summarize_text,
    utils::extract_id::extract_video_id,
};
use axum::Json;
use yt_transcript_rs::YouTubeTranscriptApi;

pub async fn summarize(Json(req): Json<SummarizeRequest>) -> String {
    let url = req.url;
    if let Some(video_id) = extract_video_id(&url) {
        if let Ok(api) = YouTubeTranscriptApi::new(None, None, None) {
            if let Ok(transcript_result) = api
                .fetch_transcript(&video_id, &["en", "es", "en-US"], false)
                .await {
                    let text = transcript_result.text();
                    summarize_text(&text).await
                } else {
                    return "Error fetching transcript".to_string();
                }
        } else {
            return "Error fetching transcript".to_string();
        }
    } else {
        "Invalid URL".to_string()
    }
}
