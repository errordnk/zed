//! Minimal stub for copilot crate
//! Copilot removed in Phase 3, terminal fork doesn't need GitHub Copilot Chat

use anyhow::Result;
use gpui::{actions, App, Entity, Task, Window};

actions!(copilot, [SignOut]);

#[derive(Clone, Debug)]
pub enum Status {
    Disabled,
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

pub fn initiate_sign_in(_window: &mut Window, _cx: &mut impl gpui::AppContext) -> Task<Result<()>> {
    Task::ready(Ok(()))
}

// Minimal types for compilation
pub mod copilot_chat {
    pub struct CopilotChat;

    impl CopilotChat {
        pub fn global(_cx: &gpui::App) -> Option<gpui::Entity<Self>> {
            None
        }
    }

    #[derive(Clone, Debug)]
    pub struct Model {
        pub id: String,
        pub display_name: String,
    }

    impl Model {
        pub fn id(&self) -> &str { &self.id }
        pub fn display_name(&self) -> &str { &self.display_name }
        pub fn max_token_count(&self) -> u64 { 4096 }
        pub fn uses_streaming(&self) -> bool { true }
    }
}

pub mod copilot_responses {}
