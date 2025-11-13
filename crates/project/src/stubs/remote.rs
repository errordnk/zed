//! Stub module for remote crate
//! The remote crate was removed in Phase 2. This stub provides minimal types for compilation.

use anyhow::Result;
use client::Client;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
}

#[derive(Clone)]
pub struct RemoteClient;

#[derive(Clone, Debug)]
pub struct RemoteConnectionOptions {
    pub ssh_connection_string: Option<String>,
}

impl RemoteClient {
    pub fn new(_client: Arc<Client>, _options: RemoteConnectionOptions) -> Self {
        Self
    }

    pub fn connection_state(&self) -> ConnectionState {
        ConnectionState::Disconnected
    }
}

#[derive(Clone, Debug)]
pub enum RemoteClientEvent {
    Disconnected,
    Connected,
}
