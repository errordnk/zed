//! Stub module for ai_onboarding crate
//! The ai_onboarding crate was removed in Phase 2. This stub provides minimal types for compilation.

use gpui::IntoElement;

pub struct YoungAccountBanner;

impl YoungAccountBanner {
    pub fn new() -> impl IntoElement {
        gpui::div()
    }
}

impl Default for YoungAccountBanner {
    fn default() -> Self {
        Self
    }
}
