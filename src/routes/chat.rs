use axum::{Json, response::IntoResponse};
use crate::models::chat::{ChatRequest, ChatResponse};
use crate::services::llm;

pub async fn chat_handler(Json(payload): Json<ChatRequest>) -> impl IntoResponse {
    let response = llm::process_chat(payload).await;
    Json(response)
}
