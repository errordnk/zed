//! Stub module for copilot crate
//! The copilot crate was removed in Phase 2. This stub provides minimal types for compilation.

use anyhow::Result;
use gpui::{actions, App, Entity, Window};
use serde::{Deserialize, Serialize};

actions!(copilot, [SignOut]);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Starting,
    SignedOut,
    SigningIn,
    Authorized,
    Unauthorized,
}

pub struct Copilot;

impl Copilot {
    pub fn global(_cx: &App) -> Option<Entity<Self>> {
        None
    }

    pub fn status(&self) -> Status {
        Status::SignedOut
    }
}

pub fn initiate_sign_in(_window: &mut Window, _cx: &mut App) -> Result<()> {
    Ok(())
}

pub mod copilot_chat {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Model {
        pub id: String,
        pub name: String,
        pub max_tokens: usize,
    }

    impl Model {
        pub fn id(&self) -> &str {
            &self.id
        }

        pub fn display_name(&self) -> &str {
            &self.name
        }

        pub fn supports_tools(&self) -> bool {
            false
        }

        pub fn supports_vision(&self) -> bool {
            false
        }

        pub fn vendor(&self) -> ModelVendor {
            ModelVendor::GitHub
        }

        pub fn max_token_count(&self) -> usize {
            self.max_tokens
        }

        pub fn tokenizer(&self) -> Option<String> {
            None
        }

        pub fn supports_response(&self) -> bool {
            false
        }

        pub fn uses_streaming(&self) -> bool {
            true
        }
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct CopilotChatConfiguration {
        pub model: Model,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum ModelVendor {
        GitHub,
        OpenAI,
        Anthropic,
        Google,
    }

    pub struct CopilotChat;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ChatMessage {
        pub role: String,
        pub content: ChatMessageContent,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum ChatMessageContent {
        Text(String),
        Parts(Vec<ChatMessagePart>),
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum ChatMessagePart {
        Text(String),
        Image(ImageUrl),
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ImageUrl {
        pub url: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Request {
        pub messages: Vec<ChatMessage>,
        pub model: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum ResponseEvent {
        Started,
        Delta(String),
        Finished,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Tool {
        pub name: String,
        pub description: String,
        pub parameters: serde_json::Value,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ToolCall {
        pub id: String,
        pub name: String,
        pub arguments: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Function {
        pub name: String,
        pub description: String,
        pub parameters: serde_json::Value,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct FunctionContent {
        pub id: String,
        pub name: String,
        pub arguments: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum ToolCallContent {
        Function { function: FunctionContent },
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum ToolChoice {
        Auto,
        Any,
        None,
    }
}

pub mod copilot_responses {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Request {
        pub messages: Vec<serde_json::Value>,
        pub model: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum ResponseOutputItem {
        Message { id: String },
        FunctionCall {
            id: String,
            name: String,
            arguments: String,
        },
        Reasoning {
            id: String,
            content: String,
        },
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum ResponseInputItem {
        Message {
            role: String,
            content: Vec<ResponseInputContent>,
        },
        FunctionCall {
            id: String,
            name: String,
            arguments: String,
        },
        FunctionCallOutput {
            call_id: String,
            output: ResponseFunctionOutput,
        },
        Reasoning {
            content: String,
        },
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum ResponseInputContent {
        InputText { text: String },
        InputImage { url: String },
        OutputText { text: String },
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum ResponseFunctionOutput {
        Text(String),
        Content(Vec<ResponseInputContent>),
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum ToolDefinition {
        Function { function: copilot_chat::Function },
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum ToolChoice {
        Auto,
        Any,
        None,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum StreamEvent {
        Created,
        OutputItemAdded {
            item: ResponseOutputItem,
        },
        OutputTextDelta {
            delta: String,
        },
        OutputItemDone {
            item: ResponseOutputItem,
        },
        Completed {
            response: Response,
        },
        Incomplete {
            response: Response,
            reason: Option<IncompleteReason>,
        },
        Failed {
            response: Response,
        },
        GenericError {
            error: String,
        },
        Unknown,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Response {
        pub id: String,
        pub output: Vec<ResponseOutputItem>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum IncompleteReason {
        MaxOutputTokens,
        ContentFilter,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum ResponseIncludable {
        ReasoningEncryptedContent,
    }
}
