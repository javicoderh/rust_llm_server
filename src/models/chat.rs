use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatRequest {
    pub message: String,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatResponse {
    pub response: String,
    pub session_id: String,
}
