//! Stub module for ai_onboarding crate
//! The ai_onboarding crate was removed in Phase 3. This stub provides minimal types for compilation.

use gpui::{IntoElement, Render, Window, Context, Element};

#[derive(Clone)]
pub struct YoungAccountBanner;

impl Render for YoungAccountBanner {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        gpui::div()
    }
}

impl IntoElement for YoungAccountBanner {
    type Element = gpui::Div;

    fn into_element(self) -> Self::Element {
        gpui::div()
    }
}
