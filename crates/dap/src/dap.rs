//! Stub module for dap (Debug Adapter Protocol) crate
//! The dap crate was removed in Phase 3. This stub provides minimal types for compilation.

use anyhow::Result;
use serde::{Deserialize, Serialize};

// Common DAP types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StackFrameId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ThreadId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BreakpointId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VariableReference(pub u64);

// Client module with SessionId
pub mod client {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct SessionId(pub u64);

    #[derive(Clone)]
    pub struct DapClient;

    impl DapClient {
        pub fn new() -> Self {
            DapClient
        }
    }
}

// Request/Response types
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InitializeRequest;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InitializeResponse;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LaunchRequest;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttachRequest;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SetBreakpointsRequest;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SetBreakpointsResponse;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContinueRequest;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StepRequest;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StackTraceRequest;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StackTraceResponse {
    pub stack_frames: Vec<StackFrame>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StackFrame {
    pub id: StackFrameId,
    pub name: String,
    pub line: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScopesRequest;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VariablesRequest;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvaluateRequest;

// Breakpoint types
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Breakpoint {
    pub id: Option<BreakpointId>,
    pub verified: bool,
    pub line: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceBreakpoint {
    pub line: u64,
    pub condition: Option<String>,
}

// Event types
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DapEvent {
    Initialized,
    Stopped { thread_id: ThreadId },
    Continued { thread_id: ThreadId },
    Exited { exit_code: i64 },
    Terminated,
    Thread { thread_id: ThreadId },
    Output { category: String, output: String },
    Breakpoint { breakpoint: Breakpoint },
}

// Stub initialization function
pub fn init() -> Result<()> {
    // DAP not needed in terminal fork
    Ok(())
}
