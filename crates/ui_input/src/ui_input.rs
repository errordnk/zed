//! This crate provides UI components that can be used for form-like scenarios, such as a input and number field.
//!
//! Note: Editor functionality has been removed in Phase 2 cleanup. Stub types are provided for compatibility.
//!

// STUB types for editor crate removed in Phase 2
mod editor_stub {
    use gpui::{App, Context, Entity, FocusHandle, Focusable, Hsla, IntoElement, Window};
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

        pub fn set_style(&mut self, _style: EditorStyle, _window: &mut Window, _cx: &mut Context<Self>) {
            // Stub - no-op
        }

        pub fn set_text(&mut self, _text: impl Into<String>, _window: &mut Window, _cx: &mut Context<Self>) {
            // Stub - no-op
        }
    }

    impl Focusable for Editor {
        fn focus_handle(&self, _cx: &App) -> FocusHandle {
            unimplemented!("Editor stub - focus_handle not implemented")
        }
    }

    impl gpui::Render for Editor {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            gpui::Empty
        }
    }

    /// Stub EditorStyle type - editor crate was removed
    #[derive(Clone)]
    pub struct EditorStyle {
        pub text: gpui::TextStyle,
        pub background: Hsla,
        pub scrollbar_width: gpui::Pixels,
        pub local_player: theme::PlayerColor,
        pub syntax: std::sync::Arc<theme::SyntaxTheme>,
    }

    impl Default for EditorStyle {
        fn default() -> Self {
            Self {
                text: gpui::TextStyle::default(),
                background: Hsla::default(),
                scrollbar_width: gpui::Pixels::default(),
                local_player: theme::PlayerColor::default(),
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

        fn id(&self) -> Option<gpui::ElementId> {
            None
        }

        fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
            None
        }

        fn request_layout(
            &mut self,
            _id: Option<&gpui::GlobalElementId>,
            _inspector_id: Option<&gpui::InspectorElementId>,
            window: &mut Window,
            cx: &mut App,
        ) -> (gpui::LayoutId, Self::RequestLayoutState) {
            // Stub - return dummy layout
            let layout_id = window.request_layout(
                gpui::Style {
                    display: gpui::Display::None,
                    ..Default::default()
                },
                None,
                cx,
            );
            (layout_id, ())
        }

        fn prepaint(
            &mut self,
            _id: Option<&gpui::GlobalElementId>,
            _inspector_id: Option<&gpui::InspectorElementId>,
            _bounds: gpui::Bounds<gpui::Pixels>,
            _request_layout: &mut Self::RequestLayoutState,
            _window: &mut Window,
            _cx: &mut App,
        ) -> Self::PrepaintState {
        }

        fn paint(
            &mut self,
            _id: Option<&gpui::GlobalElementId>,
            _inspector_id: Option<&gpui::InspectorElementId>,
            _bounds: gpui::Bounds<gpui::Pixels>,
            _request_layout: &mut Self::RequestLayoutState,
            _prepaint: &mut Self::PrepaintState,
            _window: &mut Window,
            _cx: &mut App,
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
