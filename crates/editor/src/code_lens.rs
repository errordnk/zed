use collections::HashMap;
use language::BufferId;
use project::CodeAction;
use ui::{Context, Window};
use workspace::Workspace;

use crate::{Editor, actions::ToggleCodeLens};

pub(super) struct CodeLensState {
    pub(super) blocks: HashMap<BufferId, Vec<()>>,
}

impl Default for CodeLensState {
    fn default() -> Self {
        Self {
            blocks: HashMap::default(),
        }
    }
}

impl Editor {
    pub(super) fn refresh_code_lenses(
        &mut self,
        _for_buffer: Option<BufferId>,
        _window: &Window,
        _cx: &mut Context<Self>,
    ) {
    }

    pub fn supports_code_lens(&self, _cx: &ui::App) -> bool {
        false
    }

    pub fn code_lens_enabled(&self) -> bool {
        false
    }

    pub fn toggle_code_lens_action(
        &mut self,
        _: &ToggleCodeLens,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    pub(super) fn toggle_code_lens(
        &mut self,
        _enable: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    pub(super) fn resolve_visible_code_lenses(&mut self, _cx: &mut Context<Self>) {}

    pub(super) fn clear_code_lenses(&mut self, _cx: &mut Context<Self>) {}
}

pub(crate) fn try_handle_client_command(
    _action: &CodeAction,
    _editor: &mut Editor,
    _workspace: &gpui::Entity<Workspace>,
    _window: &mut Window,
    _cx: &mut Context<Editor>,
) -> bool {
    false
}
