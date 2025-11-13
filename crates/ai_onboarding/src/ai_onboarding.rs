//! Stub module for ai_onboarding crate
//! The ai_onboarding crate was removed in Phase 2. This stub provides minimal types for compilation.

use gpui::IntoElement;

#[derive(Clone, Copy)]
pub struct YoungAccountBanner;

impl YoungAccountBanner {
    pub fn new() -> Self {
        Self
    }
}

impl Default for YoungAccountBanner {
    fn default() -> Self {
        Self
    }
}

impl IntoElement for YoungAccountBanner {
    type Element = gpui::Div;

    fn into_element(self) -> Self::Element {
        gpui::div()
    }
}
