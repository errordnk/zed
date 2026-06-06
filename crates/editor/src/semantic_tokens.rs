use gpui::{App, Context};
use settings::Settings as _;
use text::BufferId;
use ui::Window;

use project::{lsp_store::RefreshForServer, project_settings::ProjectSettings};

use crate::{Editor, actions::ToggleSemanticHighlights};

pub(super) struct SemanticTokenState {
    enabled: bool,
}

impl SemanticTokenState {
    pub(super) fn new(cx: &App, enabled: bool) -> Self {
        let _ = ProjectSettings::get_global(cx);
        Self { enabled }
    }

    pub(super) fn enabled(&self) -> bool {
        self.enabled
    }

    pub(super) fn invalidate_buffer(&mut self, _buffer_id: &BufferId) {}

    pub(super) fn update_rules(
        &mut self,
        _new_rules: settings::SemanticTokenRules,
    ) -> bool {
        false
    }
}

impl Editor {
    pub fn supports_semantic_tokens(&self, _cx: &mut App) -> bool {
        false
    }

    pub fn semantic_highlights_enabled(&self) -> bool {
        self.semantic_token_state.enabled()
    }

    pub fn toggle_semantic_highlights(
        &mut self,
        _: &ToggleSemanticHighlights,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    pub(super) fn invalidate_semantic_tokens(&mut self, _for_buffer: Option<BufferId>) {}

    pub(super) fn refresh_semantic_tokens(
        &mut self,
        _for_buffer: Option<BufferId>,
        _refresh_for_server: Option<RefreshForServer>,
        _cx: &mut Context<Self>,
    ) {
    }
}
