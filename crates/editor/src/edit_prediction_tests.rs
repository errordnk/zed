use language;
use edit_prediction_types::{
    EditPredictionDelegate, EditPredictionDiscardReason, EditPredictionIconSet,
    SuggestionDisplayType,
};
use gpui::{App, Context, Entity};
use ui::IconName;
use std::sync::{
    Arc,
    atomic::{self, AtomicUsize},
};

#[derive(Default)]
pub struct FakeEditPredictionDelegate {
    pub completion: Option<edit_prediction_types::EditPrediction>,
    pub refresh_count: Arc<AtomicUsize>,
}

impl FakeEditPredictionDelegate {
    pub fn set_edit_prediction(
        &mut self,
        completion: Option<edit_prediction_types::EditPrediction>,
    ) {
        self.completion = completion;
    }
}

impl EditPredictionDelegate for FakeEditPredictionDelegate {
    fn name() -> &'static str {
        "fake-completion-provider"
    }

    fn display_name() -> &'static str {
        "Fake Completion Provider"
    }

    fn show_predictions_in_menu() -> bool {
        true
    }

    fn supports_jump_to_edit() -> bool {
        true
    }

    fn icons(&self, _cx: &App) -> EditPredictionIconSet {
        EditPredictionIconSet::new(IconName::ZedPredict)
    }

    fn is_enabled(
        &self,
        _buffer: &Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        _cx: &App,
    ) -> bool {
        true
    }

    fn is_refreshing(&self, _cx: &App) -> bool {
        false
    }

    fn refresh(
        &mut self,
        _buffer: Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        _debounce: bool,
        _cx: &mut Context<Self>,
    ) {
        self.refresh_count.fetch_add(1, atomic::Ordering::SeqCst);
    }

    fn accept(&mut self, _cx: &mut Context<Self>) {}

    fn discard(&mut self, _reason: EditPredictionDiscardReason, _cx: &mut Context<Self>) {
        self.completion.take();
    }

    fn did_show(&mut self, _display_type: SuggestionDisplayType, _cx: &mut Context<Self>) {}

    fn suggest(
        &mut self,
        _buffer: &Entity<language::Buffer>,
        _cursor_position: language::Anchor,
        _cx: &mut Context<Self>,
    ) -> Option<edit_prediction_types::EditPrediction> {
        self.completion.clone()
    }
}
