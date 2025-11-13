//! Stub module for edit_prediction crate
//! The edit_prediction crate was removed in Phase 2. This stub provides minimal types for compilation.

use gpui::{App, Context, Entity};
use language::{Anchor, Buffer};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum Direction {
    Next,
    Prev,
}

#[derive(Clone, Debug)]
pub enum DataCollectionState {
    Enabled,
    Disabled,
}

#[derive(Clone, Debug)]
pub enum EditPrediction {
    Local {
        id: Option<String>,
        edits: Vec<(std::ops::Range<Anchor>, Arc<str>)>,
        edit_preview: Option<String>,
    },
}

pub trait EditPredictionProvider: Sized {
    fn name() -> &'static str;
    fn display_name() -> &'static str;
    fn show_completions_in_menu() -> bool {
        false
    }
    fn show_tab_accept_marker() -> bool {
        false
    }
    fn supports_jump_to_edit() -> bool {
        false
    }
    fn is_enabled(&self, _buffer: &Entity<Buffer>, _cursor_position: Anchor, _cx: &App) -> bool {
        false
    }
    fn is_refreshing(&self) -> bool {
        false
    }
    fn refresh(
        &mut self,
        _buffer_handle: Entity<Buffer>,
        _cursor_position: Anchor,
        _debounce: bool,
        _cx: &mut Context<Self>,
    ) {
    }
    fn cycle(
        &mut self,
        _buffer: Entity<Buffer>,
        _cursor_position: Anchor,
        _direction: Direction,
        _cx: &mut Context<Self>,
    ) {
    }
    fn accept(&mut self, _cx: &mut Context<Self>) {}
    fn discard(&mut self, _cx: &mut Context<Self>) {}
    fn suggest(
        &mut self,
        _buffer: &Entity<Buffer>,
        _cursor_position: Anchor,
        _cx: &mut Context<Self>,
    ) -> Option<EditPrediction> {
        None
    }
}
