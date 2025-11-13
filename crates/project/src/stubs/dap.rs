//! Stub module for dap crate (Debug Adapter Protocol)
//! The dap crate was removed in Phase 2. This stub provides minimal types for compilation.

use serde::{Deserialize, Serialize};

pub mod client {
    pub type SessionId = usize;
    pub struct DebugAdapterClient;
}

pub mod inline_value {
    #[derive(Clone, Debug)]
    pub struct InlineValueLocation;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum VariableLookupKind {
        Indexed,
        Named,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum VariableScope {
        Local,
        Global,
    }
}

pub mod requests {
    pub trait Request {
        type Arguments;
        type Response;
    }

    pub struct Initialize;
    pub struct Terminate;
    pub struct Disconnect;
    pub struct ConfigurationDone;
    pub struct Launch;
    pub struct Attach;
    pub struct Threads;
    pub struct StackTrace;
    pub struct Scopes;
    pub struct Variables;
    pub struct Evaluate;
    pub struct SetVariable;
    pub struct Completions;
    pub struct SetBreakpoints;
    pub struct SetDataBreakpoints;
    pub struct SetExceptionBreakpoints;
    pub struct DataBreakpointInfo;
    pub struct Modules;
    pub struct LoadedSources;
    pub struct Locations;
    pub struct ReadMemory;
    pub struct WriteMemory;
    pub struct StepIn;
    pub struct StepOut;
    pub struct StepBack;
    pub struct Continue;
    pub struct Pause;
    pub struct TerminateThreads;
    pub struct Restart;
    pub struct RestartFrame;
    pub struct RunInTerminal;
    pub struct StartDebugging;
}

pub mod messages {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Request {
        pub seq: i64,
        pub command: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Response {
        pub request_seq: i64,
        pub success: bool,
        pub command: String,
    }
}

pub mod adapters {
    #[derive(Clone, Debug)]
    pub struct DebugAdapterName(pub String);

    #[derive(Clone, Debug)]
    pub struct DebugAdapterBinary {
        pub path: String,
        pub args: Vec<String>,
    }

    pub trait DapDelegate {}
}

pub mod transport {
    pub struct TcpTransport;

    impl TcpTransport {
        pub async fn unused_port(_host: &str) -> anyhow::Result<u16> {
            Ok(9229) // Stub port
        }
    }
}

pub type StackFrameId = i64;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Source {
    pub name: Option<String>,
    pub path: Option<String>,
    pub source_reference: Option<i64>,
    pub presentation_hint: Option<String>,
    pub origin: Option<String>,
    pub sources: Option<Vec<Source>>,
    pub adapter_data: Option<serde_json::Value>,
    pub checksums: Option<Vec<Checksum>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Checksum {
    pub algorithm: String,
    pub checksum: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StackFrame {
    pub id: i64,
    pub name: String,
    pub source: Option<Source>,
    pub line: i64,
    pub column: i64,
    pub end_line: Option<i64>,
    pub end_column: Option<i64>,
    pub can_restart: Option<bool>,
    pub instruction_pointer_reference: Option<String>,
    pub module_id: Option<serde_json::Value>,
    pub presentation_hint: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Scope {
    pub name: String,
    pub presentation_hint: Option<String>,
    pub variables_reference: i64,
    pub named_variables: Option<i64>,
    pub indexed_variables: Option<i64>,
    pub expensive: bool,
    pub source: Option<Source>,
    pub line: Option<i64>,
    pub column: Option<i64>,
    pub end_line: Option<i64>,
    pub end_column: Option<i64>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Thread {
    pub id: i64,
    pub name: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub value: String,
    pub presentation_hint: Option<VariablePresentationHint>,
    pub evaluate_name: Option<String>,
    pub variables_reference: i64,
    pub named_variables: Option<i64>,
    pub indexed_variables: Option<i64>,
    pub memory_reference: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct VariablePresentationHint {
    pub kind: Option<String>,
    pub attributes: Option<Vec<String>>,
    pub visibility: Option<String>,
    pub lazy: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Breakpoint {
    pub id: Option<i64>,
    pub verified: bool,
    pub message: Option<String>,
    pub source: Option<Source>,
    pub line: Option<i64>,
    pub column: Option<i64>,
    pub end_line: Option<i64>,
    pub end_column: Option<i64>,
    pub instruction_reference: Option<String>,
    pub offset: Option<i64>,
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DataBreakpoint {
    pub data_id: String,
    pub access_type: Option<String>,
    pub condition: Option<String>,
    pub hit_condition: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Module {
    pub id: serde_json::Value,
    pub name: String,
    pub path: Option<String>,
    pub is_optimized: Option<bool>,
    pub is_user_code: Option<bool>,
    pub version: Option<String>,
    pub symbol_status: Option<String>,
    pub symbol_file_path: Option<String>,
    pub date_time_stamp: Option<String>,
    pub address_range: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct OutputEvent {
    pub category: Option<String>,
    pub output: String,
    pub group: Option<String>,
    pub variables_reference: Option<i64>,
    pub source: Option<Source>,
    pub line: Option<i64>,
    pub column: Option<i64>,
    pub data: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub format: String,
    pub variables: Option<std::collections::HashMap<String, String>>,
    pub send_telemetry: Option<bool>,
    pub show_user: Option<bool>,
    pub url: Option<String>,
    pub url_label: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: Option<Message>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EvaluateResponse {
    pub result: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub presentation_hint: Option<VariablePresentationHint>,
    pub variables_reference: i64,
    pub named_variables: Option<i64>,
    pub indexed_variables: Option<i64>,
    pub memory_reference: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EvaluateArguments {
    pub expression: String,
    pub frame_id: Option<i64>,
    pub context: Option<String>,
    pub format: Option<ValueFormat>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ValueFormat {
    pub hex: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ThreadsArgument {}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ConfigurationDoneArguments {}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LaunchRequestArguments {
    pub no_debug: Option<bool>,
    #[serde(rename = "__restart")]
    pub restart: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AttachRequestArguments {
    #[serde(rename = "__restart")]
    pub restart: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SetBreakpointsArguments {
    pub source: Source,
    pub breakpoints: Option<Vec<SourceBreakpoint>>,
    pub lines: Option<Vec<i64>>,
    pub source_modified: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SourceBreakpoint {
    pub line: i64,
    pub column: Option<i64>,
    pub condition: Option<String>,
    pub hit_condition: Option<String>,
    pub log_message: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DataBreakpointInfoResponse {
    pub data_id: Option<String>,
    pub description: String,
    pub access_types: Option<Vec<String>>,
    pub can_persist: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DataBreakpointInfoArguments {
    pub variables_reference: Option<i64>,
    pub name: String,
    pub frame_id: Option<i64>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SetDataBreakpointsArguments {
    pub breakpoints: Vec<DataBreakpoint>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SetExceptionBreakpointsArguments {
    pub filters: Vec<String>,
    pub filter_options: Option<Vec<ExceptionFilterOptions>>,
    pub exception_options: Option<Vec<ExceptionOptions>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ExceptionFilterOptions {
    pub filter_id: String,
    pub condition: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ExceptionOptions {
    pub path: Option<Vec<ExceptionPathSegment>>,
    pub break_mode: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ExceptionPathSegment {
    pub negate: Option<bool>,
    pub names: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LocationsResponse {
    pub body: LocationsResponseBody,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LocationsResponseBody {
    pub sources: Vec<Source>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LocationsArguments {
    pub source: Source,
    pub line: i64,
    pub column: Option<i64>,
    pub end_line: Option<i64>,
    pub end_column: Option<i64>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReadMemoryArguments {
    pub memory_reference: String,
    pub offset: Option<i64>,
    pub count: i64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WriteMemoryArguments {
    pub memory_reference: String,
    pub offset: Option<i64>,
    pub allow_partial: Option<bool>,
    pub data: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WriteMemoryResponse {
    pub offset: Option<i64>,
    pub bytes_written: Option<i64>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TerminateArguments {
    pub restart: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DisconnectArguments {
    pub restart: Option<bool>,
    pub terminate_debuggee: Option<bool>,
    pub suspend_debuggee: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RunInTerminalResponse {
    pub process_id: Option<i64>,
    pub shell_process_id: Option<i64>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CompletionItem {
    pub label: String,
    pub text: Option<String>,
    pub sort_text: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub start: Option<i64>,
    pub length: Option<i64>,
    pub selection_start: Option<i64>,
    pub selection_length: Option<i64>,
}

// Additional types and enums used in project

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EvaluateArgumentsContext {
    Watch,
    Repl,
    Hover,
    Clipboard,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BreakpointEventReason {
    Changed,
    New,
    Removed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreadEventReason {
    Started,
    Exited,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModuleEventReason {
    New,
    Changed,
    Removed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StartDebuggingRequestArgumentsRequest {
    Launch,
    Attach,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CompletionsResponse {
    pub targets: Vec<CompletionItem>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StepBackArguments {
    pub thread_id: i64,
    pub single_thread: Option<bool>,
    pub granularity: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PauseArguments {
    pub thread_id: i64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TerminateThreadsArguments {
    pub thread_ids: Option<Vec<i64>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RestartArguments {
    pub arguments: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct VariablesArguments {
    pub variables_reference: i64,
    pub filter: Option<String>,
    pub start: Option<i64>,
    pub count: Option<i64>,
    pub format: Option<ValueFormat>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SetVariableArguments {
    pub variables_reference: i64,
    pub name: String,
    pub value: String,
    pub format: Option<ValueFormat>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RestartFrameArguments {
    pub frame_id: i64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ModulesArguments {
    pub start_module: Option<i64>,
    pub module_count: Option<i64>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LoadedSourcesArguments {}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StackTraceArguments {
    pub thread_id: i64,
    pub start_frame: Option<i64>,
    pub levels: Option<i64>,
    pub format: Option<StackFrameFormat>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StackFrameFormat {
    pub parameters: Option<bool>,
    pub parameter_types: Option<bool>,
    pub parameter_names: Option<bool>,
    pub parameter_values: Option<bool>,
    pub line: Option<bool>,
    pub module: Option<bool>,
    pub include_all: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ScopesArguments {
    pub frame_id: i64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CompletionsArguments {
    pub frame_id: Option<i64>,
    pub text: String,
    pub column: i64,
    pub line: Option<i64>,
}

// Stub ProtoConversion trait for compatibility
pub trait ProtoConversion: Sized {
    fn from_proto(value: impl Into<Self>) -> anyhow::Result<Self> {
        Ok(value.into())
    }
    fn to_proto(&self) -> Self where Self: Clone {
        self.clone()
    }
}

// Implement for all DAP types
impl ProtoConversion for Source {}
impl ProtoConversion for Module {}
impl ProtoConversion for StackFrame {}
impl ProtoConversion for Scope {}
impl ProtoConversion for Thread {}
impl ProtoConversion for Variable {}
impl ProtoConversion for Breakpoint {}
impl ProtoConversion for DataBreakpoint {}

// DapRegistry and DapLocator stubs
pub struct DapRegistry;

pub trait DapLocator {}

pub struct DebugRequest;
