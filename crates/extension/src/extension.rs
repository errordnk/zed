//! Stub module for extension crate
//! The extension crate was removed in Phase 2. This stub provides minimal types for compilation.

use gpui::{App, Entity};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct ExtensionHostProxy;

impl ExtensionHostProxy {
    pub fn default_global(_cx: &App) -> Entity<Self> {
        unimplemented!("ExtensionHostProxy is stubbed")
    }

    pub fn register_context_server_proxy<T>(&self, _proxy: T) {
        // Stub - extensions not needed in terminal fork
    }
}

pub trait Extension {}

pub struct ExtensionSnippetProxy;

pub struct ContextServerConfiguration;

pub trait ExtensionContextServerProxy {}

pub trait ProjectDelegate {
    fn worktree_ids(&self) -> Vec<u64>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TargetConfig {
    pub archive: String,
    pub cmd: String,
    pub args: Vec<String>,
    pub sha256: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub name: String,
}

pub enum ExtensionEvents {
    ExtensionInstalled,
}

pub struct ExtensionSlashCommandProxy;

pub trait WorktreeDelegate {}
