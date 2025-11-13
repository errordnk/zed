//! Minimal debugger stub module
//! Debugger removed in Phase 3 - these stubs allow project crate to compile

use anyhow::Result;
use gpui::{App, Entity, EventEmitter};
use std::sync::Arc;

// Breakpoint store stubs
pub mod breakpoint_store {
    use super::*;
    use crate::{BufferStore, WorktreeStore};
    use rpc::ProtoClient;

    #[derive(Clone, Copy, Debug)]
    pub struct BreakpointWithPosition;

    #[derive(Clone, Debug)]
    pub struct ActiveStackFrame;

    pub struct BreakpointStore;

    impl BreakpointStore {
        pub fn init<C: ProtoClient + 'static>(_client: &Arc<C>) {}

        pub fn local(_worktree_store: Entity<WorktreeStore>, _buffer_store: Entity<BufferStore>) -> Self {
            Self
        }

        pub fn remote<C: ProtoClient + 'static>(_project_id: u64, _client: Arc<C>) -> Self {
            Self
        }
    }

    impl EventEmitter<()> for BreakpointStore {}
}

// DAP store stubs
pub mod dap_store {
    use super::*;
    use crate::WorktreeStore;
    use client::Client;
    use rpc::ProtoClient;

    #[derive(Clone)]
    pub enum DapStoreEvent {
        Notification(String),
    }

    pub struct DapStore;

    impl DapStore {
        pub fn init<C: ProtoClient + 'static>(_client: &Arc<C>, _cx: &mut App) {}

        pub fn new_local(_worktree_store: Entity<WorktreeStore>, _client: Arc<Client>, _cx: &mut App) -> Entity<Self> {
            panic!("Debugger not available in terminal fork")
        }

        pub fn new_remote(
            _remote_id: u64,
            _client: Arc<Client>,
            _worktree_store: Entity<WorktreeStore>,
            _cx: &mut App,
        ) -> Entity<Self> {
            panic!("Debugger not available in terminal fork")
        }

        pub fn new_collab<C: ProtoClient + 'static>(
            _project_id: u64,
            _client: Arc<C>,
            _worktree_store: Entity<WorktreeStore>,
            _cx: &mut App,
        ) -> Result<Entity<Self>> {
            panic!("Debugger not available in terminal fork")
        }
    }

    impl EventEmitter<DapStoreEvent> for DapStore {}
}

// Session stubs
pub mod session {
    use super::*;
    use language::Buffer;

    pub struct Session;

    impl Session {
        pub fn start_inline_assist(
            &self,
            _session: Entity<Self>,
            _active_stack_frame: breakpoint_store::ActiveStackFrame,
            _buffer: Entity<Buffer>,
            _cx: &mut App,
        ) -> gpui::Task<Result<()>> {
            gpui::Task::ready(Ok(()))
        }
    }

    impl EventEmitter<()> for Session {}
}
