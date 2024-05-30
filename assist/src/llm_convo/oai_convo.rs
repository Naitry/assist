use crate::llm_convo::LLMConvo;
use crate::llm_convo::LLMConvoMethods;
use async_trait::async_trait;
use reqwest::Client;
use std::collections::HashMap;

// Define the OAIConvo1_3_8 struct
pub struct OAIConvo3_0_0 {
    pub llm_convo: LLMConvo,
    client: Client,
    model: String,
    api_key: String,
}

impl OAIConvo3_0_0 {
    pub fn new(client: Client, api_key: String, model: String) -> Self {
        OAIConvo3_0_0 {
            llm_convo: LLMConvo::new(),
            client,
            model,
            api_key,
        }
    }
}

#[async_trait]
impl LLMConvoMethods for OAIConvo3_0_0 {
    async fn add_system_message(&mut self, message: String) {
        self.llm_convo.messages.push({
            let mut msg = HashMap::new();
            msg.insert("role".to_string(), "system".to_string());
            msg.insert("content".to_string(), message);
            msg.insert("DT".to_string(), self.llm_convo.current_date_time());
            msg
        });
    }

    async fn add_assistant_message(&mut self, message: String) {
        self.llm_convo.messages.push({
            let mut msg = HashMap::new();
            msg.insert("role".to_string(), "assistant".to_string());
            msg.insert("content".to_string(), message);
            msg.insert("DT".to_string(), self.llm_convo.current_date_time());
            msg
        });
    }

    async fn add_user_message(&mut self, message: String) {
        self.llm_convo.messages.push({
            let mut msg = HashMap::new();
            msg.insert("role".to_string(), "user".to_string());
            msg.insert("content".to_string(), message);
            msg.insert("DT".to_string(), self.llm_convo.current_date_time());
            msg
        });
    }

    fn formatted_messages(&self) -> Vec<HashMap<String, String>> {
        self.llm_convo
            .messages
            .iter()
            .map(|msg| {
                let mut formatted_msg = HashMap::new();
                formatted_msg.insert("role".to_string(), msg["role"].clone());
                formatted_msg.insert("content".to_string(), msg["content"].clone());
                formatted_msg
            })
            .collect()
    }

    async fn request_response(&mut self, add_to_convo: bool, max_tokens: usize) -> String {
        let request_body = serde_json::json!({
            "model": self.model,
            "messages": self.formatted_messages(),
            "temperature": 1,
            "max_tokens": max_tokens,
            "top_p": 1,
            "frequency_penalty": 0,
            "presence_penalty": 0,
        });

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await
            .unwrap()
            .json::<HashMap<String, serde_json::Value>>()
            .await
            .unwrap();

        let assistant_message = response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap()
            .to_string();

        if add_to_convo {
            self.add_assistant_message(assistant_message.clone()).await;
        }

        assistant_message
    }
}
