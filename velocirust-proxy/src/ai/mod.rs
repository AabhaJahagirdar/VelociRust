use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum AIIntent {
    Autocomplete, // Quick, local-first
    Refactor,     // High-reasoning, cloud-first
    Explain,      // High-reasoning, cloud-first
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIRequest {
    pub intent: AIIntent,
    pub file_content: String,
    pub cursor_offset: usize,
    pub file_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIResponse {
    pub text: String,
    pub is_local: bool,
}