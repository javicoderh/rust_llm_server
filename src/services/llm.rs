use crate::models::chat::{ChatRequest, ChatResponse};
use crate::models::gemini::{GeminiRequest, Content, Part, GeminiResponse};
use std::env;
use dotenvy::dotenv;

const SYSTEM_PROMPT: &str = "You are a helpful and concise assistant. Please format your response using Markdown. Use paragraphs and lists where appropriate to ensure readability.";

pub async fn process_chat(req: ChatRequest) -> ChatResponse {
    dotenv().ok();
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-flash-latest:generateContent?key={}",
        api_key
    );

    let gemini_req = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part { text: format!("{}\n\nUser: {}", SYSTEM_PROMPT, req.message) }],
            role: Some("user".to_string()),
        }],
    };

    let client = reqwest::Client::new();
    let res = client.post(&url)
        .json(&gemini_req)
        .send()
        .await;

    match res {
        Ok(response) => {
            let status = response.status();
            if status.is_success() {
                let gemini_res: Result<GeminiResponse, _> = response.json().await;
                match gemini_res {
                    Ok(parsed) => {
                        if let Some(candidates) = parsed.candidates {
                            if let Some(first) = candidates.first() {
                                if let Some(part) = first.content.parts.first() {
                                    return ChatResponse { response: part.text.clone() };
                                }
                            }
                        }
                        ChatResponse { response: "No content generated".to_string() }
                    }
                    Err(e) => ChatResponse { response: format!("Failed to parse response: {}", e) }
                }
            } else {
                let error_text = response.text().await.unwrap_or_default();
                ChatResponse { response: format!("API Error: {} - Body: {}", status, error_text) }
            }
        }
        Err(e) => ChatResponse { response: format!("Request failed: {}", e) }
    }
}
