// REMOVED: extension crate deleted in Phase 2
// Theme extensions are not needed in terminal fork - only built-in themes

use std::sync::Arc;
use gpui::BackgroundExecutor;
use theme::ThemeRegistry;

// Stub types for deleted extension crate
pub struct ExtensionHostProxy;

/// Stub init function - extension system removed
/// Theme loading works through built-in theme registry only
pub fn init(
    _extension_host_proxy: Arc<ExtensionHostProxy>,
    theme_registry: Arc<ThemeRegistry>,
    _executor: BackgroundExecutor,
) {
    // No-op: extension themes not supported
    // Built-in themes work through ThemeRegistry directly
    theme_registry.set_extensions_loaded();
}
