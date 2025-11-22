use serde::Deserialize;

#[derive(Deserialize)]
pub struct SummarizeRequest {
    pub url: String,
}
