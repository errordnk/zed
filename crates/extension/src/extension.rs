//! Stub module for extension crate
//! The extension crate was removed in Phase 2. This stub provides minimal types for compilation.

use collections::HashMap;
use gpui::{App, Entity};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct ExtensionHostProxy;

impl ExtensionHostProxy {
    pub fn default_global(_cx: &App) -> Entity<Self> {
        unimplemented!("ExtensionHostProxy is stubbed")
    }
}

// Extension trait to add methods to Entity<ExtensionHostProxy>
pub trait ExtensionHostProxyExt {
    fn register_context_server_proxy<T: ExtensionContextServerProxy>(&self, _proxy: T);
}

impl ExtensionHostProxyExt for Entity<ExtensionHostProxy> {
    fn register_context_server_proxy<T: ExtensionContextServerProxy>(&self, _proxy: T) {
        // Stub - extensions not needed in terminal fork
    }
}

pub trait Extension: Send + Sync {
    fn context_server_command(
        &self,
        _id: Arc<str>,
        _project: Arc<dyn ProjectDelegate>,
    ) -> gpui::Task<anyhow::Result<ContextServerCommand>> {
        gpui::Task::ready(Err(anyhow::anyhow!("Not implemented")))
    }

    fn path_from_extension(&self, path: &str) -> String {
        path.to_string()
    }

    fn context_server_configuration(
        &self,
        _id: Arc<str>,
        _project: Arc<dyn ProjectDelegate>,
    ) -> gpui::Task<anyhow::Result<Option<ContextServerConfiguration>>> {
        gpui::Task::ready(Ok(None))
    }
}

#[derive(Clone, Debug)]
pub struct ContextServerCommand {
    pub command: String,
    pub args: Vec<String>,
    pub env: std::collections::HashMap<String, String>,
}

pub struct ExtensionSnippetProxy;

#[derive(Clone, Debug)]
pub struct ContextServerConfiguration;

pub trait ExtensionContextServerProxy: Send + Sync + 'static {
    fn register_context_server(
        &self,
        extension: Arc<dyn Extension>,
        id: Arc<str>,
        cx: &mut App,
    );
    fn unregister_context_server(&self, server_id: Arc<str>, cx: &mut App);
}

pub trait ProjectDelegate {
    fn worktree_ids(&self) -> Vec<u64>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub name: String,
    #[serde(default)]
    pub agent_servers: std::collections::BTreeMap<Arc<str>, AgentServerManifestEntry>,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct AgentServerManifestEntry {
    pub name: String,
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(default)]
    pub icon: Option<String>,
    pub targets: HashMap<String, TargetConfig>,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct TargetConfig {
    pub archive: String,
    pub cmd: String,
    #[serde(default)]
    pub args: Vec<String>,
    pub sha256: Option<String>,
}

pub enum ExtensionEvents {
    ExtensionInstalled,
}

impl ExtensionEvents {
    pub fn try_global(_cx: &App) -> Option<&'static Self> {
        None
    }
}

pub struct ExtensionSlashCommandProxy;

pub trait WorktreeDelegate {}
