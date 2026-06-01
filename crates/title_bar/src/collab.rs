use gpui::{AnyElement, App, IntoElement, Window};
use crate::TitleBar;

pub fn toggle_screen_sharing(_cx: &mut App) {}

pub fn toggle_mute(_cx: &mut App) {}

pub fn toggle_deafen(_cx: &mut App) {}

impl TitleBar {
    pub(crate) fn render_collaborator_list(
        &self,
        _window: &mut Window,
        _cx: &mut gpui::Context<Self>,
    ) -> AnyElement {
        gpui::Empty.into_any_element()
    }

    pub(crate) fn render_call_controls(
        &self,
        _window: &mut Window,
        _cx: &mut gpui::Context<Self>,
    ) -> Vec<AnyElement> {
        Vec::new()
    }
}
