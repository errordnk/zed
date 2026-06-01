use gpui::{AnyElement, App, WeakEntity};
use workspace::Workspace;

use super::QuickActionBar;

impl QuickActionBar {
    pub fn render_preview_button(
        &self,
        _workspace_handle: WeakEntity<Workspace>,
        _cx: &mut App,
    ) -> Option<AnyElement> {
        None
    }
}
