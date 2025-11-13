//! Stub module for copilot crate
//! The copilot crate was removed in Phase 3. This stub provides minimal types for compilation.

use anyhow::Result;
use gpui::{actions, Window};
use serde::{Deserialize, Serialize};

actions!(copilot, [SignOut]);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Disabled,
    SignedOut,
    SigningIn,
    SignedIn,
    Error,
}

#[derive(Clone)]
pub struct Copilot;

impl Copilot {
    pub fn status(&self) -> Status {
        Status::Disabled
    }
}

pub fn initiate_sign_in(_window: Window, _cx: &mut impl gpui::AppContext) -> impl std::future::Future<Output = Result<()>> {
    async { Ok(()) }
}

// copilot_chat module
pub mod copilot_chat {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Model {
        pub name: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct CopilotChatConfiguration {
        pub model: Model,
        pub messages: Vec<Message>,
        pub tools: Vec<Function>,
        pub tool_choice: ToolChoice,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Message {
        pub role: String,
        pub content: String,
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
    #[serde(tag = "type")]
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

// copilot_responses module
pub mod copilot_responses {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Request {
        pub model: String,
        pub messages: Vec<Message>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Message {
        pub role: String,
        pub content: String,
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
        Reasoning { content: String },
    }

    #[derive(Clone, Debug)]
    pub struct Response {
        pub id: String,
        pub output: Vec<ResponseOutputItem>,
    }

    #[derive(Clone, Debug)]
    pub struct IncompleteResponse {
        pub id: String,
        pub incomplete_reason: Option<IncompleteReason>,
    }

    #[derive(Clone, Debug)]
    pub enum IncompleteReason {
        MaxOutputTokens,
        ContentFilter,
    }

    #[derive(Clone, Debug)]
    pub enum ResponseIncludable {
        ReasoningEncryptedContent,
    }
}
