//! Stub module for dap crate (Debug Adapter Protocol)
//! The dap crate was removed in Phase 2. This stub provides minimal types for compilation.

use serde::{Deserialize, Serialize};
use serde_json::Value;

// Re-export for nested modules
pub use self::{client::*, inline_value::*, requests::*};

pub mod client {
    pub struct DebugAdapterClient;
}

pub mod inline_value {
    #[derive(Clone, Debug)]
    pub struct InlineValueLocation;

    #[derive(Clone, Debug)]
    pub enum VariableLookupKind {
        Indexed,
        Named,
    }

    #[derive(Clone, Debug)]
    pub enum VariableScope {
        Local,
        Global,
    }
}

pub mod requests {
    pub struct Terminate;
    pub struct Disconnect;
}

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
