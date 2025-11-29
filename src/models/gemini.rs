use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GeminiRequest {
    pub contents: Vec<Content>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub parts: Vec<Part>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Part {
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct GeminiResponse {
    pub candidates: Option<Vec<Candidate>>,
}

#[derive(Deserialize, Debug)]
pub struct Candidate {
    pub content: Content,
    pub finishReason: Option<String>,
}
