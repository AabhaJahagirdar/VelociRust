use adk_gemini::Gemini;
use futures_util::TryStreamExt;
use crate::ai::{AIRequest, AIIntent, AIResponse};

pub struct AIRouter {
    gemini_client: Gemini,
    local_url: String,
}

impl AIRouter {
    pub fn new(api_key: String) -> Self {
        Self {
            gemini_client: Gemini::new(api_key).unwrap(),
            local_url: "http://localhost:11434/api/generate".to_string(), // Ollama/uzu default
        }
    }

    pub async fn handle_request(&self, req: AIRequest) -> Result<(), Box<dyn std::error::Error>> {
        match req.intent {
            AIIntent::Autocomplete => {
                // ROUTE TO LOCAL: Fast ghost-text
                self.stream_local(req).await?;
            }
            _ => {
                // ROUTE TO CLOUD: Complex reasoning via Gemini 3.1 Flash-Lite
                self.stream_cloud(req).await?;
            }
        }
        Ok(())
    }

    async fn stream_cloud(&self, req: AIRequest) -> Result<(), Box<dyn std::error::Error>> {
        let mut stream = self.gemini_client
            .generate_content()
            .with_system_prompt("You are the VelociRust AI. Provide concise code completions.")
            .with_user_message(format!("File: {}\nContent: {}", req.file_path, req.file_content))
            .execute_stream()
            .await?;

        while let Some(chunk) = stream.try_next().await? {
            // Send this to the VelociRust UI via SSE
            println!("TOKEN: {}", chunk.text());
        }
        Ok(())
    }

    async fn stream_local(&self, req: AIRequest) -> Result<(), Box<dyn std::error::Error>> {
        // Here we point to your Local 1.5B model for instant feedback
        // Implementation for local streaming via 'uzu' or 'ollama' goes here
        Ok(())
    }
}