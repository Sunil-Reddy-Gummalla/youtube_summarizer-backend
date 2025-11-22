use yt_transcript_rs::YouTubeTranscriptApi;
use axum::{Json};
use crate::{models::summarize_request::SummarizeRequest, utils::extract_id::extract_video_id, services::llm::summarize_text};



pub async fn summarize(Json(req): Json<SummarizeRequest>) -> String {
    let url = req.url;
    println!("recieved url: {url}");
    let video_id = extract_video_id(&url) 
        .expect("Failed to extract video id");
    println!("video id: {video_id}");
    let api = YouTubeTranscriptApi::new(None, None, None).expect("failed to get youtube transcript api");
    let transcript_result = api.fetch_transcript(
        &video_id,
        &["en", "es", "en-US"], 
        false
    ).await.expect("failed to fetch the transcript");
    let text = transcript_result.text();
    let summerized_text = summarize_text(&text).await;
    summerized_text
}