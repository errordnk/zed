//! This crate provides UI components that can be used for form-like scenarios, such as a input and number field.
//!
//! Note: Editor functionality has been removed in Phase 2 cleanup. Stub types are provided for compatibility.
//!

// STUB types for editor crate removed in Phase 2
mod editor_stub {
    use gpui::{App, Context, Entity, FocusHandle, Focusable, Hsla, Window};
    use ui::SharedString;

    /// Stub Editor type - editor crate was removed
    pub struct Editor;

    impl Editor {
        pub fn single_line(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
            Self
        }

        pub fn set_placeholder_text(&mut self, _text: &SharedString, _window: &mut Window, _cx: &mut Context<Self>) {
            // Stub - no-op
        }

        pub fn text(&self, _cx: &App) -> String {
            String::new()
        }

        pub fn set_read_only(&mut self, _read_only: bool) {
            // Stub - no-op
        }
    }

    impl Focusable for Editor {
        fn focus_handle(&self, _cx: &App) -> FocusHandle {
            unimplemented!("Editor stub - focus_handle not implemented")
        }
    }

    /// Stub EditorStyle type - editor crate was removed
    #[derive(Clone)]
    pub struct EditorStyle {
        pub text: gpui::TextStyle,
        pub background: Hsla,
        pub scrollbar_width: gpui::Pixels,
        pub local_player: theme::PlayerColors,
        pub syntax: std::sync::Arc<theme::SyntaxTheme>,
    }

    impl Default for EditorStyle {
        fn default() -> Self {
            Self {
                text: gpui::TextStyle::default(),
                background: Hsla::default(),
                scrollbar_width: gpui::Pixels::default(),
                local_player: theme::PlayerColors::default(),
                syntax: std::sync::Arc::new(theme::SyntaxTheme::default()),
            }
        }
    }

    /// Stub EditorElement type - editor crate was removed
    pub struct EditorElement;

    impl EditorElement {
        pub fn new(_editor: &Entity<Editor>, _style: EditorStyle) -> Self {
            Self
        }
    }

    impl gpui::IntoElement for EditorElement {
        type Element = Self;
        fn into_element(self) -> Self::Element {
            self
        }
    }

    impl gpui::Element for EditorElement {
        type RequestLayoutState = ();
        type PrepaintState = ();

        fn request_layout(
            &mut self,
            _id: Option<&gpui::GlobalElementId>,
            _cx: &mut gpui::WindowContext,
        ) -> (gpui::LayoutId, Self::RequestLayoutState) {
            // Stub - return dummy layout
            (gpui::LayoutId::default(), ())
        }

        fn prepaint(
            &mut self,
            _id: Option<&gpui::GlobalElementId>,
            _bounds: gpui::Bounds<gpui::Pixels>,
            _request_layout: &mut Self::RequestLayoutState,
            _cx: &mut gpui::WindowContext,
        ) -> Self::PrepaintState {
        }

        fn paint(
            &mut self,
            _id: Option<&gpui::GlobalElementId>,
            _bounds: gpui::Bounds<gpui::Pixels>,
            _request_layout: &mut Self::RequestLayoutState,
            _prepaint: &mut Self::PrepaintState,
            _cx: &mut gpui::WindowContext,
        ) {
            // Stub - no painting
        }
    }
}

// Re-export stub types as if they were from editor crate
pub use editor_stub::{Editor, EditorElement, EditorStyle};

mod input_field;
mod number_field;

pub use input_field::*;
pub use number_field::*;
