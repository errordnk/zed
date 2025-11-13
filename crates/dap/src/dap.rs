//! Stub module for dap (Debug Adapter Protocol) crate
//! The dap crate was removed in Phase 3. This stub provides comprehensive types for compilation.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;

// Common DAP ID types
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

// Capabilities
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Capabilities {
    pub supports_configuration_done_request: Option<bool>,
    pub supports_function_breakpoints: Option<bool>,
    pub supports_conditional_breakpoints: Option<bool>,
    pub supports_hit_conditional_breakpoints: Option<bool>,
    pub supports_evaluate_for_hovers: Option<bool>,
    pub supports_step_back: Option<bool>,
    pub supports_set_variable: Option<bool>,
    pub supports_restart_frame: Option<bool>,
    pub supports_goto_targets_request: Option<bool>,
    pub supports_step_in_targets_request: Option<bool>,
    pub supports_completions_request: Option<bool>,
    pub supports_modules_request: Option<bool>,
    pub supports_restart_request: Option<bool>,
    pub supports_exception_options: Option<bool>,
    pub supports_value_formatting_options: Option<bool>,
    pub supports_exception_info_request: Option<bool>,
    pub supports_terminate_threads_request: Option<bool>,
    pub supports_delayed_stack_trace_loading: Option<bool>,
    pub supports_loaded_sources_request: Option<bool>,
    pub supports_log_points: Option<bool>,
    pub supports_terminate_debuggee: Option<bool>,
    pub supports_suspend_debuggee: Option<bool>,
    pub supports_clipboard_context: Option<bool>,
    pub supports_stepping_granularity: Option<bool>,
    pub supports_instruction_breakpoints: Option<bool>,
    pub supports_exception_filter_options: Option<bool>,
    pub supports_data_breakpoints: Option<bool>,
    pub supports_read_memory_request: Option<bool>,
    pub supports_write_memory_request: Option<bool>,
    pub supports_disassemble_request: Option<bool>,
    pub exception_breakpoint_filters: Option<Vec<ExceptionBreakpointsFilter>>,
}

// Initialize request arguments
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InitializeRequestArguments {
    pub client_id: Option<String>,
    pub client_name: Option<String>,
    pub adapter_id: String,
    pub locale: Option<String>,
    pub lines_start_at1: Option<bool>,
    pub columns_start_at1: Option<bool>,
    pub path_format: Option<InitializeRequestArgumentsPathFormat>,
    pub supports_variable_type: Option<bool>,
    pub supports_variable_paging: Option<bool>,
    pub supports_run_in_terminal_request: Option<bool>,
    pub supports_memory_references: Option<bool>,
    pub supports_progress_reporting: Option<bool>,
    pub supports_invalidated_event: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InitializeRequestArgumentsPathFormat {
    Path,
    Uri,
}

// Argument types for various requests
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContinueArguments {
    pub thread_id: ThreadId,
    pub single_thread: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NextArguments {
    pub thread_id: ThreadId,
    pub single_thread: Option<bool>,
    pub granularity: Option<SteppingGranularity>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StepInArguments {
    pub thread_id: ThreadId,
    pub single_thread: Option<bool>,
    pub target_id: Option<u64>,
    pub granularity: Option<SteppingGranularity>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StepOutArguments {
    pub thread_id: ThreadId,
    pub single_thread: Option<bool>,
    pub granularity: Option<SteppingGranularity>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SteppingGranularity {
    Statement,
    Line,
    Instruction,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VariablesArgumentsFilter {
    Indexed,
    Named,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EvaluateArgumentsContext {
    Watch,
    Repl,
    Hover,
    Clipboard,
    Variables,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WriteMemoryArguments {
    pub memory_reference: String,
    pub offset: Option<i64>,
    pub allow_partial: Option<bool>,
    pub data: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RunInTerminalRequestArguments {
    pub kind: Option<String>,
    pub title: Option<String>,
    pub cwd: String,
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StartDebuggingRequestArguments {
    pub configuration: HashMap<String, Value>,
    pub request: StartDebuggingRequestArgumentsRequest,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StartDebuggingRequestArgumentsRequest {
    Launch,
    Attach,
}

// Response types
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SetVariableResponse {
    pub value: String,
    pub type_: Option<String>,
    pub variables_reference: Option<VariableReference>,
    pub named_variables: Option<u64>,
    pub indexed_variables: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvaluateResponse {
    pub result: String,
    pub type_: Option<String>,
    pub presentation_hint: Option<VariablePresentationHint>,
    pub variables_reference: VariableReference,
    pub named_variables: Option<u64>,
    pub indexed_variables: Option<u64>,
    pub memory_reference: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompletionsResponse {
    pub targets: Vec<CompletionItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompletionItem {
    pub label: String,
    pub text: Option<String>,
    pub sort_text: Option<String>,
    pub detail: Option<String>,
    pub type_: Option<String>,
    pub start: Option<u64>,
    pub length: Option<u64>,
    pub selection_start: Option<u64>,
    pub selection_length: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataBreakpointInfoResponse {
    pub data_id: Option<String>,
    pub description: String,
    pub access_types: Option<Vec<String>>,
    pub can_persist: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataBreakpoint {
    pub data_id: String,
    pub access_type: Option<String>,
    pub condition: Option<String>,
    pub hit_condition: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocationsResponse {
    pub body: LocationsResponseBody,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocationsResponseBody {
    pub locations: Vec<Location>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    pub line: u64,
    pub column: Option<u64>,
    pub end_line: Option<u64>,
    pub end_column: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WriteMemoryResponse {
    pub offset: Option<i64>,
    pub bytes_written: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RunInTerminalResponse {
    pub process_id: Option<u64>,
    pub shell_process_id: Option<u64>,
}

// Core types
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StackFrame {
    pub id: StackFrameId,
    pub name: String,
    pub source: Option<Source>,
    pub line: u64,
    pub column: u64,
    pub end_line: Option<u64>,
    pub end_column: Option<u64>,
    pub can_restart: Option<bool>,
    pub instruction_pointer_reference: Option<String>,
    pub module_id: Option<Value>,
    pub presentation_hint: Option<StackFramePresentationHint>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StackFramePresentationHint {
    Normal,
    Label,
    Subtle,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Source {
    pub name: Option<String>,
    pub path: Option<PathBuf>,
    pub source_reference: Option<u64>,
    pub presentation_hint: Option<String>,
    pub origin: Option<String>,
    pub sources: Option<Vec<Source>>,
    pub adapter_data: Option<Value>,
    pub checksums: Option<Vec<Checksum>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Checksum {
    pub algorithm: String,
    pub checksum: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Thread {
    pub id: ThreadId,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Scope {
    pub name: String,
    pub presentation_hint: Option<String>,
    pub variables_reference: VariableReference,
    pub named_variables: Option<u64>,
    pub indexed_variables: Option<u64>,
    pub expensive: bool,
    pub source: Option<Source>,
    pub line: Option<u64>,
    pub column: Option<u64>,
    pub end_line: Option<u64>,
    pub end_column: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub value: String,
    pub type_: Option<String>,
    pub presentation_hint: Option<VariablePresentationHint>,
    pub evaluate_name: Option<String>,
    pub variables_reference: VariableReference,
    pub named_variables: Option<u64>,
    pub indexed_variables: Option<u64>,
    pub memory_reference: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VariablePresentationHint {
    pub kind: Option<String>,
    pub attributes: Option<Vec<String>>,
    pub visibility: Option<String>,
    pub lazy: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Module {
    pub id: Value,
    pub name: String,
    pub path: Option<PathBuf>,
    pub is_optimized: Option<bool>,
    pub is_user_code: Option<bool>,
    pub version: Option<String>,
    pub symbol_status: Option<String>,
    pub symbol_file_path: Option<PathBuf>,
    pub date_time_stamp: Option<String>,
    pub address_range: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Breakpoint {
    pub id: Option<BreakpointId>,
    pub verified: bool,
    pub message: Option<String>,
    pub source: Option<Source>,
    pub line: Option<u64>,
    pub column: Option<u64>,
    pub end_line: Option<u64>,
    pub end_column: Option<u64>,
    pub instruction_reference: Option<String>,
    pub offset: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceBreakpoint {
    pub line: u64,
    pub column: Option<u64>,
    pub condition: Option<String>,
    pub hit_condition: Option<String>,
    pub log_message: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExceptionBreakpointsFilter {
    pub filter: String,
    pub label: String,
    pub description: Option<String>,
    pub default: Option<bool>,
    pub supports_condition: Option<bool>,
    pub condition_description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExceptionFilterOptions {
    pub filter_id: String,
    pub condition: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValueFormat {
    pub hex: Option<bool>,
}

// Event types
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputEvent {
    pub category: Option<OutputEventCategory>,
    pub output: String,
    pub group: Option<String>,
    pub variables_reference: Option<VariableReference>,
    pub source: Option<Source>,
    pub line: Option<u64>,
    pub column: Option<u64>,
    pub data: Option<Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OutputEventCategory {
    Console,
    Stdout,
    Stderr,
    Telemetry,
    Important,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BreakpointEventReason {
    Changed,
    New,
    Removed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ThreadEventReason {
    Started,
    Exited,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ModuleEventReason {
    New,
    Changed,
    Removed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DapEvent {
    Initialized,
    Stopped { thread_id: ThreadId, reason: String },
    Continued { thread_id: ThreadId },
    Exited { exit_code: i64 },
    Terminated,
    Thread { thread_id: ThreadId, reason: ThreadEventReason },
    Output(OutputEvent),
    Breakpoint { reason: BreakpointEventReason, breakpoint: Breakpoint },
    Module { reason: ModuleEventReason, module: Module },
}

// Error types
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: Option<Message>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub format: String,
    pub variables: Option<HashMap<String, String>>,
    pub send_telemetry: Option<bool>,
    pub show_user: Option<bool>,
    pub url: Option<String>,
    pub url_label: Option<String>,
}

// Request/Response infrastructure
#[derive(Clone, Debug)]
pub struct DebugRequest;

#[derive(Clone)]
pub struct DapRegistry;

impl DapRegistry {
    pub fn new() -> Self {
        DapRegistry
    }
}

#[derive(Clone)]
pub struct DapLocator;

// Module stubs
pub mod requests {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct InitializeRequest {
        pub arguments: InitializeRequestArguments,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct LaunchRequest {
        pub arguments: HashMap<String, Value>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct AttachRequest {
        pub arguments: HashMap<String, Value>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct SetBreakpointsRequest {
        pub source: Source,
        pub breakpoints: Option<Vec<SourceBreakpoint>>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ContinueRequest {
        pub arguments: ContinueArguments,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct NextRequest {
        pub arguments: NextArguments,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct StepInRequest {
        pub arguments: StepInArguments,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct StepOutRequest {
        pub arguments: StepOutArguments,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct StackTraceRequest {
        pub thread_id: ThreadId,
        pub start_frame: Option<u64>,
        pub levels: Option<u64>,
        pub format: Option<ValueFormat>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ScopesRequest {
        pub frame_id: StackFrameId,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct VariablesRequest {
        pub variables_reference: VariableReference,
        pub filter: Option<VariablesArgumentsFilter>,
        pub start: Option<u64>,
        pub count: Option<u64>,
        pub format: Option<ValueFormat>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct EvaluateRequest {
        pub expression: String,
        pub frame_id: Option<StackFrameId>,
        pub context: Option<EvaluateArgumentsContext>,
        pub format: Option<ValueFormat>,
    }
}

pub mod messages {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Request {
        pub seq: u64,
        pub command: String,
        pub arguments: Option<Value>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Response {
        pub seq: u64,
        pub request_seq: u64,
        pub success: bool,
        pub command: String,
        pub message: Option<String>,
        pub body: Option<Value>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Event {
        pub seq: u64,
        pub event: String,
        pub body: Option<Value>,
    }
}

pub mod adapters {
    use super::*;

    pub trait Adapter {
        fn name(&self) -> &str;
    }

    #[derive(Clone)]
    pub struct GenericAdapter;

    impl Adapter for GenericAdapter {
        fn name(&self) -> &str {
            "generic"
        }
    }
}

pub mod transport {
    use super::*;

    pub trait Transport {
        fn send(&self, data: &[u8]) -> Result<()>;
        fn receive(&self) -> Result<Vec<u8>>;
    }

    #[derive(Clone)]
    pub struct StdioTransport;

    impl Transport for StdioTransport {
        fn send(&self, _data: &[u8]) -> Result<()> {
            Ok(())
        }

        fn receive(&self) -> Result<Vec<u8>> {
            Ok(Vec::new())
        }
    }
}

pub mod inline_value {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct InlineValue {
        pub text: String,
        pub range: Range,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Range {
        pub start: Position,
        pub end: Position,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Position {
        pub line: u64,
        pub character: u64,
    }
}

pub mod proto_conversions {
    use super::*;

    pub fn thread_id_to_proto(id: ThreadId) -> u64 {
        id.0
    }

    pub fn thread_id_from_proto(id: u64) -> ThreadId {
        ThreadId(id)
    }

    pub fn stack_frame_id_to_proto(id: StackFrameId) -> u64 {
        id.0
    }

    pub fn stack_frame_id_from_proto(id: u64) -> StackFrameId {
        StackFrameId(id)
    }

    pub fn breakpoint_id_to_proto(id: BreakpointId) -> u64 {
        id.0
    }

    pub fn breakpoint_id_from_proto(id: u64) -> BreakpointId {
        BreakpointId(id)
    }

    pub fn variable_reference_to_proto(id: VariableReference) -> u64 {
        id.0
    }

    pub fn variable_reference_from_proto(id: u64) -> VariableReference {
        VariableReference(id)
    }
}

// Responses
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InitializeResponse {
    pub body: Option<Capabilities>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SetBreakpointsResponse {
    pub breakpoints: Vec<Breakpoint>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StackTraceResponse {
    pub stack_frames: Vec<StackFrame>,
    pub total_frames: Option<u64>,
}

// Stub initialization function
pub fn init() -> Result<()> {
    // DAP not needed in terminal fork
    Ok(())
}
