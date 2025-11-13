//! Stub module for extension_host crate
//! The extension_host crate was removed in Phase 2. This stub provides minimal types for compilation.

use gpui::App;
use std::sync::atomic::AtomicBool;

pub mod wasm_host {
    use super::*;

    thread_local! {
        pub static IS_WASM_THREAD: AtomicBool = const { AtomicBool::new(false) };
    }
}

#[derive(Clone)]
pub struct ExtensionStore;

impl ExtensionStore {
    pub fn global(_cx: &App) -> gpui::Model<Self> {
        unimplemented!("ExtensionStore is stubbed")
    }
}

pub struct ExtensionOperation;

pub fn init(
    _fs: std::sync::Arc<dyn util::fs::Fs>,
    _client: std::sync::Arc<client::Client>,
    _node_runtime: std::sync::Arc<node_runtime::NodeRuntime>,
    _language_registry: std::sync::Arc<language::LanguageRegistry>,
    _theme_registry: std::sync::Arc<theme::ThemeRegistry>,
    _cx: &mut App,
) {
    // Stub - extensions not needed in terminal fork
}
