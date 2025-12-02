use crate::models::chat::{ChatRequest, ChatResponse};
use crate::models::gemini::{GeminiRequest, Content, Part, GeminiResponse};
use crate::state::AppState;
use std::env;
use dotenvy::dotenv;
use uuid::Uuid;

const SYSTEM_PROMPT: &str = "You are a helpful and concise assistant. Please format your response using Markdown. Use paragraphs and lists where appropriate to ensure readability.";

pub async fn process_chat(req: ChatRequest, state: AppState) -> ChatResponse {
    dotenv().ok();
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-flash-latest:generateContent?key={}",
        api_key
    );

    let session_id = req.session_id.unwrap_or_else(|| Uuid::new_v4().to_string());

    let mut history = {
        let conversations = state.conversations.read().unwrap();
        conversations.get(&session_id).cloned().unwrap_or_else(Vec::new)
    };

    // Add user message to history
    let user_content = Content {
        parts: vec![Part { text: req.message.clone() }],
        role: Some("user".to_string()),
    };
    history.push(user_content);

    // Construct Gemini request with system prompt + history
    // Note: Gemini doesn't support system prompt in the same way as OpenAI in the `contents` array directly as "system".
    // We prepend it to the first user message or just rely on the context.
    // For simplicity and consistency with previous logic, we'll prepend it to the *current* message if history is empty,
    // or just rely on the model's ability to follow instructions if we send it every time?
    // Actually, sending it every time in the first message is better.
    // But since we are sending the whole history, we need to be careful.
    // Let's prepend SYSTEM_PROMPT to the *first* message in the history if it's the start.
    // Or simpler: Just prepend it to the current message text if we are constructing the request.
    
    // Wait, if we send history, we send a list of Content objects.
    // We should probably inject the system prompt into the first message of the history.
    
    let mut request_contents = history.clone();
    if let Some(first) = request_contents.first_mut() {
        if let Some(part) = first.parts.first_mut() {
            if !part.text.contains(SYSTEM_PROMPT) {
                 part.text = format!("{}\n\n{}", SYSTEM_PROMPT, part.text);
            }
        }
    }

    let gemini_req = GeminiRequest {
        contents: request_contents,
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
                                    let bot_response = part.text.clone();
                                    
                                    // Add bot response to history
                                    let bot_content = Content {
                                        parts: vec![Part { text: bot_response.clone() }],
                                        role: Some("model".to_string()),
                                    };
                                    
                                    {
                                        let mut conversations = state.conversations.write().unwrap();
                                        let mut current_history = conversations.get(&session_id).cloned().unwrap_or_else(Vec::new);
                                        // We need to add the user message we added locally, and the bot response
                                        // Actually, we already added user message to `history` local var, but not to state.
                                        // Let's re-fetch or just append.
                                        // Better: just append to the state vector.
                                        
                                        // Re-read state to be safe or just update.
                                        // Since we hold the lock, we can update.
                                        // Wait, I dropped the read lock.
                                        
                                        current_history.push(Content {
                                            parts: vec![Part { text: req.message }],
                                            role: Some("user".to_string()),
                                        });
                                        current_history.push(bot_content);
                                        conversations.insert(session_id.clone(), current_history);
                                    }

                                    return ChatResponse { 
                                        response: bot_response,
                                        session_id,
                                    };
                                }
                            }
                        }
                        ChatResponse { response: "No content generated".to_string(), session_id }
                    }
                    Err(e) => ChatResponse { response: format!("Failed to parse response: {}", e), session_id }
                }
            } else {
                let error_text = response.text().await.unwrap_or_default();
                ChatResponse { response: format!("API Error: {} - Body: {}", status, error_text), session_id }
            }
        }
        Err(e) => ChatResponse { response: format!("Request failed: {}", e), session_id }
    }
}
