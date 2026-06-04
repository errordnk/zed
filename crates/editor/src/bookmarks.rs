use ui::{Context, Window};
use workspace::Workspace;

use crate::{
    Editor, GoToNextBookmark, GoToPreviousBookmark, ToggleBookmark, ViewBookmarks,
    display_map::DisplayRow,
};

impl Editor {
    pub fn set_show_bookmarks(&mut self, show_bookmarks: bool, cx: &mut Context<Self>) {
        self.show_bookmarks = Some(show_bookmarks);
        cx.notify();
    }

    pub fn toggle_bookmark(
        &mut self,
        _: &ToggleBookmark,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    pub fn toggle_bookmark_at_row(&mut self, _row: DisplayRow, _cx: &mut Context<Self>) {}

    pub fn go_to_next_bookmark(
        &mut self,
        _: &GoToNextBookmark,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    pub fn go_to_previous_bookmark(
        &mut self,
        _: &GoToPreviousBookmark,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    pub fn view_bookmarks(
        _workspace: &mut Workspace,
        _: &ViewBookmarks,
        _window: &mut Window,
        _cx: &mut Context<Workspace>,
    ) {
    }

    pub fn toggle_bookmark_at_anchor(
        &mut self,
        _anchor: multi_buffer::Anchor,
        _cx: &mut Context<Self>,
    ) {
    }
}
