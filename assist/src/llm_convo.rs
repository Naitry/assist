use async_trait::async_trait;
use chrono::prelude::*;
use std::collections::HashMap;
pub mod oai_convo;
pub mod generate_tools_json;

// Define a struct for LLMConvo
pub struct LLMConvo {
    pub messages: Vec<HashMap<String, String>>,
}

#[allow(dead_code)]
impl LLMConvo {
    pub fn new() -> Self {
        LLMConvo {
            messages: Vec::new(),
        }
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }

    pub fn current_date_time(&self) -> String {
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

// Define a trait for the abstract methods
#[async_trait]
pub trait LLMConvoMethods {
    async fn add_system_message(&mut self, message: String);
    async fn add_assistant_message(&mut self, message: String);
    async fn add_user_message(&mut self, message: String);
    fn formatted_messages(&self) -> Vec<HashMap<String, String>>;
    async fn request_response(&mut self, add_to_convo: bool, max_tokens: usize) -> String;
}
