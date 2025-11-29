use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatResponse {
    pub response: String,
}
