//! Stub module for remote crate
//! The remote crate was removed in Phase 2. This stub provides minimal types for compilation.

use client::Client;
use collections::HashMap;
use rpc::AnyProtoClient;
use std::path::PathBuf;
use std::sync::Arc;
use task::SpawnInTerminal;

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

    pub fn proto_client(&self) -> AnyProtoClient {
        panic!("Remote client not available in terminal fork")
    }

    pub fn build_command(
        &self,
        _program: Option<String>,
        _args: &[String],
        _env: &HashMap<String, String>,
        _root_dir: Option<String>,
        _login: Option<SpawnInTerminal>,
    ) -> anyhow::Result<SpawnInTerminal> {
        Ok(SpawnInTerminal {
            command: Some(String::new()),
            args: Vec::new(),
            env: HashMap::default(),
            cwd: PathBuf::new(),
            label: String::new(),
            ..Default::default()
        })
    }
}

#[derive(Clone, Debug)]
pub enum RemoteClientEvent {
    Disconnected,
    Connected,
}
