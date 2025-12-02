use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::models::gemini::Content;

#[derive(Clone)]
pub struct AppState {
    pub conversations: Arc<RwLock<HashMap<String, Vec<Content>>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            conversations: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
