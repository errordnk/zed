use collections::HashMap;
use gpui::{App, Context, Pixels};
use language::language_settings::{InlayHintKind, InlayHintSettings};
use lsp::LanguageServerId;
use multi_buffer::Anchor;
use text::BufferId;
use ui::Window;

use super::InlayId;
use crate::{Editor, ToggleInlayHints, ToggleInlineValues};

pub fn inlay_hint_settings(
    location: Anchor,
    snapshot: &multi_buffer::MultiBufferSnapshot,
    cx: &mut Context<Editor>,
) -> InlayHintSettings {
    snapshot.language_settings_at(location, cx).inlay_hints
}

#[derive(Debug)]
pub struct LspInlayHintData {
    pub added_hints: HashMap<InlayId, Option<InlayHintKind>>,
    enabled: bool,
}

impl LspInlayHintData {
    pub fn new(settings: InlayHintSettings) -> Self {
        Self {
            added_hints: HashMap::default(),
            enabled: settings.enabled,
        }
    }

    pub fn modifiers_override(&mut self, _new_override: bool) -> Option<bool> {
        None
    }

    pub fn toggle(&mut self, _enabled: bool) -> bool {
        false
    }

    pub fn clear(&mut self) {
        self.added_hints.clear();
    }

    pub fn clear_for_buffers<'a>(
        &mut self,
        _removed_buffer_ids: impl IntoIterator<Item = &'a BufferId> + 'a,
    ) {
    }

    pub(crate) fn remove_inlay_chunk_data<'a>(
        &mut self,
        _removed_buffer_ids: impl IntoIterator<Item = &'a BufferId> + 'a,
    ) {
    }
}

#[derive(Debug, Clone)]
pub enum InlayHintRefreshReason {
    ModifiersChanged(bool),
    Toggle(bool),
    SettingsChange(InlayHintSettings),
    NewLinesShown,
    BufferEdited(BufferId),
    ServerRemoved,
    RefreshRequested {
        server_id: LanguageServerId,
        request_id: Option<usize>,
    },
    BuffersRemoved(Vec<BufferId>),
}

impl Editor {
    pub fn supports_inlay_hints(&self, _cx: &mut App) -> bool {
        false
    }

    pub fn toggle_inline_values(
        &mut self,
        _: &ToggleInlineValues,
        _: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    pub fn toggle_inlay_hints(
        &mut self,
        _: &ToggleInlayHints,
        _: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    pub fn inlay_hints_enabled(&self) -> bool {
        false
    }

    pub(crate) fn refresh_inlay_hints(
        &mut self,
        _reason: InlayHintRefreshReason,
        _cx: &mut Context<Self>,
    ) {
    }

    pub fn clear_inlay_hints(&mut self, _cx: &mut Context<Self>) {}

    pub fn update_inlay_link_and_hover_points(
        &mut self,
        _snapshot: &crate::EditorSnapshot,
        _point_for_position: crate::PointForPosition,
        _mouse_position: Option<gpui::Point<Pixels>>,
        _command_held: bool,
        _shift_held: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}
