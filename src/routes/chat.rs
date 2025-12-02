use axum::{Json, response::IntoResponse, extract::State};
use crate::models::chat::{ChatRequest, ChatResponse};
use crate::services::llm;
use crate::state::AppState;

pub async fn chat_handler(
    State(state): State<AppState>,
    Json(payload): Json<ChatRequest>
) -> impl IntoResponse {
    let response = llm::process_chat(payload, state).await;
    Json(response)
}
