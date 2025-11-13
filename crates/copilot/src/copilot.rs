//! Stub module for copilot crate
//! The copilot crate was removed in Phase 3. This stub provides comprehensive types for compilation.

use anyhow::Result;
use gpui::{actions, App, Entity, Task, Window};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

actions!(copilot, [SignOut]);

#[derive(Clone, Debug)]
pub enum Status {
    Disabled,
    Starting {
        task: Arc<Task<()>>,
    },
    SignedOut {
        awaiting_signing_in: bool,
    },
    SigningIn {
        prompt: Option<String>,
    },
    Authorized,
    Unauthorized,
    SignedIn,
    Error(Arc<anyhow::Error>),
}

#[derive(Clone)]
pub struct Copilot;

impl Copilot {
    pub fn global(_cx: &App) -> Option<Entity<Self>> {
        None
    }

    pub fn status(&self) -> Status {
        Status::Disabled
    }
}

pub fn initiate_sign_in(_window: &mut Window, _cx: &mut impl gpui::AppContext) -> impl std::future::Future<Output = Result<()>> {
    async { Ok(()) }
}

// copilot_chat module
pub mod copilot_chat {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Model {
        pub name: String,
        pub id: String,
        pub display_name: String,
        pub vendor: ModelVendor,
        pub max_tokens: usize,
    }

    impl Model {
        pub fn id(&self) -> &str {
            &self.id
        }

        pub fn display_name(&self) -> &str {
            &self.display_name
        }

        pub fn supports_tools(&self) -> bool {
            true
        }

        pub fn supports_vision(&self) -> bool {
            false
        }

        pub fn vendor(&self) -> ModelVendor {
            self.vendor
        }

        pub fn max_token_count(&self) -> usize {
            self.max_tokens
        }

        pub fn tokenizer(&self) -> Option<Arc<tokenizers::Tokenizer>> {
            None
        }

        pub fn supports_response(&self) -> bool {
            true
        }

        pub fn uses_streaming(&self) -> bool {
            true
        }
    }

    #[derive(Clone, Copy, Debug, Serialize, Deserialize)]
    pub enum ModelVendor {
        OpenAI,
        Anthropic,
        Google,
        GitHub,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct CopilotChatConfiguration {
        pub model: Model,
        pub messages: Vec<ChatMessage>,
        pub tools: Vec<Tool>,
        pub tool_choice: ToolChoice,
        pub enterprise_uri: Option<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ChatMessage {
        pub role: String,
        pub content: ChatMessageContent,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum ChatMessageContent {
        Text(String),
        Parts(Vec<ChatMessagePart>),
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(tag = "type")]
    pub enum ChatMessagePart {
        #[serde(rename = "text")]
        Text { text: String },
        #[serde(rename = "image_url")]
        ImageUrl { image_url: ImageUrl },
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ImageUrl {
        pub url: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Tool {
        pub function: Function,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Function {
        pub name: String,
        pub description: String,
        pub parameters: serde_json::Value,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct FunctionContent {
        pub name: String,
        pub arguments: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ToolCall {
        pub id: String,
        pub function: FunctionContent,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum ToolChoice {
        Auto,
        Any,
        None,
        Specific { function: FunctionChoice },
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct FunctionChoice {
        pub name: String,
    }

    #[derive(Clone, Debug)]
    pub enum ResponseEvent {
        Started,
        MessageDelta { delta: String },
        ToolCallStarted { tool_call: ToolCall },
        ToolCallDelta { delta: String },
        Finished,
        Error { error: String },
    }

    #[derive(Clone)]
    pub struct CopilotChat;

    // Alias for Request
    pub type Request = super::copilot_responses::Request;
}

// copilot_responses module
pub mod copilot_responses {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Request {
        pub model: String,
        pub messages: Vec<RequestMessage>,
        pub input: Option<Vec<ResponseInputItem>>,
        pub stream: Option<bool>,
        pub temperature: Option<f32>,
        pub tools: Option<Vec<ToolDefinition>>,
        pub tool_choice: Option<ToolChoice>,
        pub reasoning: Option<ReasoningConfig>,
        pub include: Option<Vec<ResponseIncludable>>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct RequestMessage {
        pub role: String,
        pub content: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ToolDefinition {
        pub name: String,
        pub description: String,
        pub parameters: serde_json::Value,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum ToolChoice {
        Auto,
        Required,
        None,
        Specific { function: FunctionChoice },
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct FunctionChoice {
        pub name: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ReasoningConfig {
        pub enabled: bool,
    }

    #[derive(Clone, Debug)]
    pub enum StreamEvent {
        Created,
        OutputItemAdded { item: ResponseOutputItem },
        OutputTextDelta { delta: String },
        OutputItemDone { item: ResponseOutputItem },
        Completed { response: Response },
        Incomplete { response: IncompleteResponse },
        Failed { response: Response },
        GenericError { error: String },
        Unknown,
    }

    #[derive(Clone, Debug)]
    pub enum ResponseOutputItem {
        Message { id: String },
        FunctionCall { id: String, name: String, arguments: Option<String> },
        Reasoning {
            content: String,
            summary: Option<String>,
            encrypted_content: Option<String>,
        },
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum ResponseInputItem {
        Message(ResponseInputContent),
        FunctionOutput(ResponseFunctionOutput),
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ResponseInputContent {
        pub role: String,
        pub content: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ResponseFunctionOutput {
        pub call_id: String,
        pub output: String,
    }

    #[derive(Clone, Debug)]
    pub struct Response {
        pub id: String,
        pub output: Vec<ResponseOutputItem>,
        pub usage: Option<Usage>,
        pub error: Option<String>,
    }

    #[derive(Clone, Debug)]
    pub struct Usage {
        pub prompt_tokens: u64,
        pub completion_tokens: u64,
        pub total_tokens: u64,
    }

    #[derive(Clone, Debug)]
    pub struct IncompleteResponse {
        pub id: String,
        pub incomplete_reason: Option<IncompleteReason>,
        pub incomplete_details: Option<IncompleteDetails>,
        pub usage: Option<Usage>,
    }

    #[derive(Clone, Debug)]
    pub struct IncompleteDetails {
        pub reason: String,
    }

    #[derive(Clone, Debug)]
    pub enum IncompleteReason {
        MaxOutputTokens,
        ContentFilter,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum ResponseIncludable {
        #[serde(rename = "reasoning_encrypted_content")]
        ReasoningEncryptedContent,
    }
}
