//! Stub module for ai_onboarding crate
//! The ai_onboarding crate was removed in Phase 2. This stub provides minimal types for compilation.

use gpui::{AnyElement, IntoElement, Render};

pub struct YoungAccountBanner;

impl YoungAccountBanner {
    pub fn new() -> Self {
        Self
    }
}

impl Default for YoungAccountBanner {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for YoungAccountBanner {
    fn render(&mut self, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        gpui::Empty
    }
}

impl IntoElement for YoungAccountBanner {
    type Element = YoungAccountBanner;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl gpui::Element for YoungAccountBanner {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<gpui::ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _cx: &mut gpui::App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        unimplemented!("YoungAccountBanner is stubbed")
    }

    fn prepaint(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _bounds: gpui::Bounds<gpui::Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _cx: &mut gpui::App,
    ) -> Self::PrepaintState {
    }

    fn paint(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _bounds: gpui::Bounds<gpui::Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        _cx: &mut gpui::App,
    ) {
    }
}
