use gpui::Hsla;
use std::ops::Range;
use text::BufferId;
use ui::{App, Context, Window};

use settings::Settings as _;

use crate::{DisplayPoint, Editor, EditorSettings, EditorSnapshot, editor_settings::DocumentColorsRenderMode};

#[derive(Debug)]
pub(super) struct LspColorData {
    pub(super) render_mode: DocumentColorsRenderMode,
}

impl LspColorData {
    pub fn new(cx: &App) -> Self {
        Self {
            render_mode: EditorSettings::get_global(cx).lsp_document_colors,
        }
    }

    pub fn render_mode_updated(
        &mut self,
        _new_mode: DocumentColorsRenderMode,
    ) -> Option<crate::inlays::InlaySplice> {
        None
    }

    pub fn editor_display_highlights(
        &self,
        _snapshot: &EditorSnapshot,
    ) -> (DocumentColorsRenderMode, Vec<(Range<DisplayPoint>, Hsla)>) {
        (self.render_mode, Vec::new())
    }
}

impl Editor {
    pub(super) fn refresh_document_colors(
        &mut self,
        _for_buffer: Option<BufferId>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}
