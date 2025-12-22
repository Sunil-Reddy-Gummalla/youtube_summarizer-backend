use crate::{
    models::{summarize_request::SummarizeRequest, summarize_response::SummarizeResponse},
    services::llm::summarize_text,
    utils::extract_id::extract_video_id,
};
use axum::{Json, http::StatusCode};
use yt_transcript_rs::YouTubeTranscriptApi;

pub async fn summarize(Json(req): Json<SummarizeRequest>) -> (StatusCode, Json<SummarizeResponse>) {
    let url = req.url;
    if let Some(video_id) = extract_video_id(&url) {
        if let Ok(api) = YouTubeTranscriptApi::new(None, None, None) {
            if let Ok(transcript_result) = api
                .fetch_transcript(&video_id, &["en", "es", "en-US"], false)
                .await
            {
                let text = transcript_result.text();
                return summarize_text(&text).await;
            } else {
                (
                    StatusCode::BAD_REQUEST,
                    Json(SummarizeResponse {
                        summary: None,
                        error: Some("Failed to fetch transcript".to_string()),
                    }),
                )
            }
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SummarizeResponse {
                    summary: None,
                    error: Some("Failed to init the Transcripter".to_string()),
                }),
            )
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(SummarizeResponse {
                summary: None,
                error: Some("Invalid URL".to_string()),
            }),
        )
    }
}
