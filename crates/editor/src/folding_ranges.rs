use text::BufferId;
use ui::{Context, Window};

use crate::Editor;

impl Editor {
    pub(super) fn refresh_folding_ranges(
        &mut self,
        _for_buffer: Option<BufferId>,
        _window: &Window,
        _cx: &mut Context<Self>,
    ) {
    }

    pub fn document_folding_ranges_enabled(&self, _cx: &ui::App) -> bool {
        false
    }

    pub(super) fn clear_disabled_lsp_folding_ranges(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}
