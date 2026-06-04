use collections::HashMap;
use gpui::{App, Entity, Task};
use language::Buffer;
use lsp::LanguageServerId;
use project::lsp_store::LspDocumentLink;
use text::BufferId;
use ui::Context;

use settings::Settings as _;

use crate::{Editor, editor_settings::EditorSettings};

pub(super) struct LspDocumentLinks {
    pub(super) enabled: bool,
    pub(super) per_buffer: HashMap<BufferId, ()>,
    pub(super) refresh_task: Task<()>,
}

impl LspDocumentLinks {
    pub(super) fn new(cx: &App) -> Self {
        Self {
            enabled: EditorSettings::get_global(cx).lsp_document_links,
            per_buffer: HashMap::default(),
            refresh_task: Task::ready(()),
        }
    }
}

impl Editor {
    pub(super) fn refresh_document_links(
        &mut self,
        _for_buffer: Option<BufferId>,
        _cx: &mut Context<Self>,
    ) {
    }

    pub fn document_links_at(
        &mut self,
        _buffer: Entity<Buffer>,
        _position: text::Anchor,
        _cx: &mut Context<Self>,
    ) -> Option<Task<Vec<(LanguageServerId, LspDocumentLink)>>> {
        None
    }
}
