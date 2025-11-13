//! Stub module for remote crate
//! The remote crate was removed in Phase 3. This stub provides minimal types for compilation.

use anyhow::Result;
use gpui::{App, Entity};

#[derive(Clone)]
pub struct RemoteClient;

impl RemoteClient {
    pub fn global(_cx: &App) -> Option<Entity<Self>> {
        // Remote connections not supported in terminal fork
        None
    }

    pub fn new() -> Self {
        RemoteClient
    }
}

#[derive(Clone)]
pub struct RemoteServer;

#[derive(Clone, Debug)]
pub struct RemoteSettings {
    pub enabled: bool,
}

impl Default for RemoteSettings {
    fn default() -> Self {
        Self { enabled: false }
    }
}

pub fn init() -> Result<()> {
    // Remote functionality not needed in terminal fork
    Ok(())
}
