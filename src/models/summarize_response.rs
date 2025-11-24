use serde::Serialize;

#[derive(Serialize)]
pub struct SummarizeResponse {
    pub summary: Option<String>,
    pub error: Option<String>,
}
