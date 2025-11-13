//! Stub module for ai_onboarding crate
//! The ai_onboarding crate was removed in Phase 3. This stub provides minimal types for compilation.

use gpui::Render;

#[derive(Clone)]
pub struct YoungAccountBanner;

impl Render for YoungAccountBanner {
    fn render(&mut self, _cx: &mut gpui::ViewContext<Self>) -> impl gpui::IntoElement {
        gpui::div()
    }
}
