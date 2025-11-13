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

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct CopilotChatConfiguration {
        pub model: Model,
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
