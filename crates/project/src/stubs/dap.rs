//! Minimal DAP (Debug Adapter Protocol) stub types
//! Debugger removed in Phase 3 - these stubs allow project crate to compile

use gpui::{App, Entity};

// Minimal inline_value types
pub mod inline_value {
    #[derive(Clone, Debug)]
    pub struct InlineValueLocation {
        pub line: u32,
        pub column: u32,
        pub scope: VariableScope,
        pub name: String,
        pub lookup: VariableLookupKind,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum VariableScope {
        Local,
        Global,
    }

    #[derive(Clone, Copy, Debug)]
    pub enum VariableLookupKind {
        Variable,
    }
}

// Minimal client types
pub mod client {
    pub struct DebugAdapterClient;
}

// Minimal adapters types
pub mod adapters {
    #[derive(Clone, Debug, Hash, PartialEq, Eq)]
    pub struct DebugAdapterName(pub String);
}

pub struct DapRegistry;

impl DapRegistry {
    pub fn global(_cx: &App) -> &'static Self {
        // Return static stub reference
        static REGISTRY: DapRegistry = DapRegistry;
        &REGISTRY
    }

    pub fn locators(&self) -> std::collections::HashMap<String, ()> {
        std::collections::HashMap::new()
    }
}
