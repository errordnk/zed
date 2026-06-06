use super::*;

pub fn make_suggestion_styles(cx: &App) -> EditPredictionStyles {
    EditPredictionStyles {
        insertion: HighlightStyle {
            color: Some(cx.theme().status().predictive),
            ..HighlightStyle::default()
        },
        whitespace: HighlightStyle {
            background_color: Some(cx.theme().status().created_background),
            ..HighlightStyle::default()
        },
    }
}

pub(super) enum EditDisplayMode {
    TabAccept,
    DiffPopover,
    Inline,
}

pub(super) enum EditPrediction {
    Edit {
        // TODO could be a language::Anchor?
        edits: Vec<(Range<Anchor>, Arc<str>)>,
        /// Predicted cursor position as (anchor, offset_from_anchor).
        /// The anchor is in multibuffer coordinates; after applying edits,
        /// resolve the anchor and add the offset to get the final cursor position.
        cursor_position: Option<(Anchor, usize)>,
        display_mode: EditDisplayMode,
    },
    /// Move to a specific location in the active editor
    MoveWithin {
        target: Anchor,
    },
    /// Move to a specific location in a different editor (not the active one)
    MoveOutside {
        target: language::Anchor,
        snapshot: BufferSnapshot,
    },
}

pub(super) struct EditPredictionState {
    pub(super) inlay_ids: Vec<InlayId>,
    pub(super) completion: EditPrediction,
    pub(super) completion_id: Option<SharedString>,
    pub(super) invalidation_range: Option<Range<Anchor>>,
}

pub(super) enum EditPredictionSettings {
    Disabled,
    Enabled {
        show_in_menu: bool,
        preview_requires_modifier: bool,
    },
}

pub(super) enum MenuEditPredictionsPolicy {
    #[cfg(test)]
    Never,
    ByProvider,
}

pub(super) enum EditPredictionPreview {
    /// Modifier is not pressed
    Inactive,
    /// Modifier pressed
    Active {
        previous_scroll_position: Option<SharedScrollAnchor>,
    },
}

impl EditPredictionPreview {
    pub(super) fn set_previous_scroll_position(
        &mut self,
        scroll_position: Option<SharedScrollAnchor>,
    ) {
        if let EditPredictionPreview::Active {
            previous_scroll_position,
            ..
        } = self
        {
            *previous_scroll_position = scroll_position;
        }
    }
}

pub(super) struct RegisteredEditPredictionDelegate {
    pub(super) provider: Arc<dyn EditPredictionDelegateHandle>,
    _subscription: Subscription,
}

impl Editor {
    pub fn set_edit_prediction_provider<T>(
        &mut self,
        provider: Option<Entity<T>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) where
        T: EditPredictionDelegate,
    {
        self.edit_prediction_provider = provider.map(|provider| RegisteredEditPredictionDelegate {
            _subscription: cx.observe_in(&provider, window, |this, _, window, cx| {
                if this.focus_handle.is_focused(window) {
                    this.update_visible_edit_prediction(window, cx);
                }
            }),
            provider: Arc::new(provider),
        });
        self.update_edit_prediction_settings(cx);
        self.refresh_edit_prediction(false, false, window, cx);
    }

    pub fn set_edit_predictions_hidden_for_vim_mode(
        &mut self,
        hidden: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if hidden != self.edit_predictions_hidden_for_vim_mode {
            self.edit_predictions_hidden_for_vim_mode = hidden;
            if hidden {
                self.update_visible_edit_prediction(window, cx);
            } else {
                self.refresh_edit_prediction(true, false, window, cx);
            }
        }
    }

    pub fn toggle_edit_predictions(
        &mut self,
        _: &ToggleEditPrediction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.show_edit_predictions_override.is_some() {
            self.set_show_edit_predictions(None, window, cx);
        } else {
            let show_edit_predictions = !self.edit_predictions_enabled();
            self.set_show_edit_predictions(Some(show_edit_predictions), window, cx);
        }
    }

    pub fn set_show_edit_predictions(
        &mut self,
        show_edit_predictions: Option<bool>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.show_edit_predictions_override = show_edit_predictions;
        self.update_edit_prediction_settings(cx);

        if let Some(false) = show_edit_predictions {
            self.discard_edit_prediction(EditPredictionDiscardReason::Ignored, cx);
        } else {
            self.refresh_edit_prediction(false, true, window, cx);
        }
    }

    pub fn refresh_edit_prediction(
        &mut self,
        debounce: bool,
        user_requested: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        if self.leader_id.is_some() {
            self.discard_edit_prediction(EditPredictionDiscardReason::Ignored, cx);
            return None;
        }

        let cursor = self.selections.newest_anchor().head();
        let (buffer, cursor_buffer_position) =
            self.buffer.read(cx).text_anchor_for_position(cursor, cx)?;

        if DisableAiSettings::is_ai_disabled_for_buffer(Some(&buffer), cx) {
            return None;
        }

        if !self.edit_predictions_enabled_in_buffer(&buffer, cursor_buffer_position, cx) {
            self.discard_edit_prediction(EditPredictionDiscardReason::Ignored, cx);
            return None;
        }

        self.update_visible_edit_prediction(window, cx);

        if !user_requested
            && (!self.should_show_edit_predictions()
                || !self.is_focused(window)
                || buffer.read(cx).is_empty())
        {
            self.discard_edit_prediction(EditPredictionDiscardReason::Ignored, cx);
            return None;
        }

        self.edit_prediction_provider()?
            .refresh(buffer, cursor_buffer_position, debounce, cx);
        Some(())
    }

    pub fn edit_predictions_enabled(&self) -> bool {
        match self.edit_prediction_settings {
            EditPredictionSettings::Disabled => false,
            EditPredictionSettings::Enabled { .. } => true,
        }
    }

    pub fn update_edit_prediction_settings(&mut self, cx: &mut Context<Self>) {
        if self.edit_prediction_provider.is_none() {
            self.edit_prediction_settings = EditPredictionSettings::Disabled;
            self.discard_edit_prediction(EditPredictionDiscardReason::Ignored, cx);
            return;
        }

        let selection = self.selections.newest_anchor();
        let cursor = selection.head();

        if let Some((buffer, cursor_buffer_position)) =
            self.buffer.read(cx).text_anchor_for_position(cursor, cx)
        {
            if DisableAiSettings::is_ai_disabled_for_buffer(Some(&buffer), cx) {
                self.edit_prediction_settings = EditPredictionSettings::Disabled;
                self.discard_edit_prediction(EditPredictionDiscardReason::Ignored, cx);
                return;
            }
            self.edit_prediction_settings =
                self.edit_prediction_settings_at_position(&buffer, cursor_buffer_position, cx);
        }
    }

    pub fn edit_prediction_preview_is_active(&self) -> bool {
        matches!(
            self.edit_prediction_preview,
            EditPredictionPreview::Active { .. }
        )
    }

    pub fn edit_predictions_enabled_at_cursor(&self, cx: &App) -> bool {
        let cursor = self.selections.newest_anchor().head();
        if let Some((buffer, cursor_position)) =
            self.buffer.read(cx).text_anchor_for_position(cursor, cx)
        {
            self.edit_predictions_enabled_in_buffer(&buffer, cursor_position, cx)
        } else {
            false
        }
    }

    pub fn show_edit_prediction(
        &mut self,
        _: &ShowEditPrediction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.has_active_edit_prediction() {
            self.refresh_edit_prediction(false, true, window, cx);
            return;
        }

        self.update_visible_edit_prediction(window, cx);
    }

    pub fn accept_partial_edit_prediction(
        &mut self,
        granularity: EditPredictionGranularity,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.read_only(cx) {
            return;
        }
        if self.show_edit_predictions_in_menu() {
            self.hide_context_menu(window, cx);
        }

        let Some(active_edit_prediction) = self.active_edit_prediction.as_ref() else {
            return;
        };

        if !matches!(granularity, EditPredictionGranularity::Full) && self.selections.count() != 1 {
            return;
        }

        match &active_edit_prediction.completion {
            EditPrediction::MoveWithin { target, .. } => {
                let target = *target;

                if matches!(granularity, EditPredictionGranularity::Full) {
                    if let Some(position_map) = &self.last_position_map {
                        let target_row = target.to_display_point(&position_map.snapshot).row();
                        let is_visible = position_map.visible_row_range.contains(&target_row);

                        if is_visible || !self.edit_prediction_requires_modifier() {
                            self.unfold_ranges(&[target..target], true, false, cx);
                            self.change_selections(
                                SelectionEffects::scroll(Autoscroll::newest()),
                                window,
                                cx,
                                |selections| {
                                    selections.select_anchor_ranges([target..target]);
                                },
                            );
                            self.clear_row_highlights::<EditPredictionPreview>();
                            self.edit_prediction_preview
                                .set_previous_scroll_position(None);
                        } else {
                            // Highlight and request scroll
                            self.edit_prediction_preview
                                .set_previous_scroll_position(Some(
                                    position_map.snapshot.scroll_anchor,
                                ));
                            self.highlight_rows::<EditPredictionPreview>(
                                target..target,
                                cx.theme().colors().editor_highlighted_line_background,
                                RowHighlightOptions {
                                    autoscroll: true,
                                    ..Default::default()
                                },
                                cx,
                            );
                            self.request_autoscroll(Autoscroll::fit(), cx);
                        }
                    }
                } else {
                    self.change_selections(
                        SelectionEffects::scroll(Autoscroll::newest()),
                        window,
                        cx,
                        |selections| {
                            selections.select_anchor_ranges([target..target]);
                        },
                    );
                }
            }
            EditPrediction::MoveOutside { snapshot, target } => {
                if let Some(workspace) = self.workspace() {
                    Self::open_editor_at_anchor(snapshot, *target, &workspace, window, cx)
                        .detach_and_log_err(cx);
                }
            }
            EditPrediction::Edit {
                edits,
                cursor_position,
                ..
            } => {
                self.report_edit_prediction_event(
                    active_edit_prediction.completion_id.clone(),
                    true,
                    cx,
                );

                match granularity {
                    EditPredictionGranularity::Full => {
                        let transaction_id_prev = self.buffer.read(cx).last_transaction_id(cx);

                        // Compute fallback cursor position BEFORE applying the edit,
                        // so the anchor tracks through the edit correctly
                        let fallback_cursor_target = {
                            let snapshot = self.buffer.read(cx).snapshot(cx);
                            let Some((last_edit_range, _)) = edits.last() else {
                                return;
                            };
                            last_edit_range.end.bias_right(&snapshot)
                        };

                        self.buffer.update(cx, |buffer, cx| {
                            buffer.edit(edits.iter().cloned(), None, cx)
                        });

                        if let Some(provider) = self.edit_prediction_provider() {
                            provider.accept(cx);
                        }

                        // Resolve cursor position after the edit is applied
                        let cursor_target = if let Some((anchor, offset)) = cursor_position {
                            // The anchor tracks through the edit, then we add the offset
                            let snapshot = self.buffer.read(cx).snapshot(cx);
                            let base_offset = anchor.to_offset(&snapshot).0;
                            let target_offset =
                                MultiBufferOffset((base_offset + offset).min(snapshot.len().0));
                            snapshot.anchor_after(target_offset)
                        } else {
                            fallback_cursor_target
                        };

                        self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                            s.select_anchor_ranges([cursor_target..cursor_target]);
                        });

                        let selections = self.selections.disjoint_anchors_arc();
                        if let Some(transaction_id_now) =
                            self.buffer.read(cx).last_transaction_id(cx)
                        {
                            if transaction_id_prev != Some(transaction_id_now) {
                                self.selection_history
                                    .insert_transaction(transaction_id_now, selections);
                            }
                        }

                        self.update_visible_edit_prediction(window, cx);
                        if self.active_edit_prediction.is_none() {
                            self.refresh_edit_prediction(true, true, window, cx);
                        }
                        cx.notify();
                    }
                    _ => {
                        let snapshot = self.buffer.read(cx).snapshot(cx);
                        let cursor_offset = self
                            .selections
                            .newest::<MultiBufferOffset>(&self.display_snapshot(cx))
                            .head();

                        let insertion = edits.iter().find_map(|(range, text)| {
                            let range = range.to_offset(&snapshot);
                            if range.is_empty() && range.start == cursor_offset {
                                Some(text)
                            } else {
                                None
                            }
                        });

                        if let Some(text) = insertion {
                            let text_to_insert = match granularity {
                                EditPredictionGranularity::Word => {
                                    let mut partial = text
                                        .chars()
                                        .by_ref()
                                        .take_while(|c| c.is_alphabetic())
                                        .collect::<String>();
                                    if partial.is_empty() {
                                        partial = text
                                            .chars()
                                            .by_ref()
                                            .take_while(|c| c.is_whitespace() || !c.is_alphabetic())
                                            .collect::<String>();
                                    }
                                    partial
                                }
                                EditPredictionGranularity::Line => {
                                    if let Some(line) = text.split_inclusive('\n').next() {
                                        line.to_string()
                                    } else {
                                        text.to_string()
                                    }
                                }
                                EditPredictionGranularity::Full => unreachable!(),
                            };

                            cx.emit(EditorEvent::InputHandled {
                                utf16_range_to_replace: None,
                                text: text_to_insert.clone().into(),
                            });

                            self.replace_selections(&text_to_insert, None, window, cx, false);
                            self.refresh_edit_prediction(true, true, window, cx);
                            cx.notify();
                        } else {
                            self.accept_partial_edit_prediction(
                                EditPredictionGranularity::Full,
                                window,
                                cx,
                            );
                        }
                    }
                }
            }
        }
    }

    pub fn accept_next_word_edit_prediction(
        &mut self,
        _: &AcceptNextWordEditPrediction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.accept_partial_edit_prediction(EditPredictionGranularity::Word, window, cx);
    }

    pub fn accept_next_line_edit_prediction(
        &mut self,
        _: &AcceptNextLineEditPrediction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.accept_partial_edit_prediction(EditPredictionGranularity::Line, window, cx);
    }

    pub fn accept_edit_prediction(
        &mut self,
        _: &AcceptEditPrediction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.accept_partial_edit_prediction(EditPredictionGranularity::Full, window, cx);
    }

    pub fn has_active_edit_prediction(&self) -> bool {
        self.active_edit_prediction.is_some()
    }

    /// Returns true when we're displaying the edit prediction popover below the cursor
    /// like we are not previewing and the LSP autocomplete menu is visible
    /// or we are in `when_holding_modifier` mode.
    pub fn edit_prediction_visible_in_cursor_popover(&self, has_completion: bool) -> bool {
        if self.edit_prediction_preview_is_active()
            || !self.show_edit_predictions_in_menu()
            || !self.edit_predictions_enabled()
        {
            return false;
        }

        if self.has_visible_completions_menu() {
            return true;
        }

        has_completion && self.edit_prediction_requires_modifier()
    }

    pub fn edit_prediction_provider(&self) -> Option<Arc<dyn EditPredictionDelegateHandle>> {
        Some(self.edit_prediction_provider.as_ref()?.provider.clone())
    }

    pub(super) fn show_edit_predictions_in_menu(&self) -> bool {
        match self.edit_prediction_settings {
            EditPredictionSettings::Disabled => false,
            EditPredictionSettings::Enabled { show_in_menu, .. } => show_in_menu,
        }
    }

    pub(super) fn edit_prediction_requires_modifier(&self) -> bool {
        match self.edit_prediction_settings {
            EditPredictionSettings::Disabled => false,
            EditPredictionSettings::Enabled {
                preview_requires_modifier,
                ..
            } => preview_requires_modifier,
        }
    }

    pub(super) fn discard_edit_prediction(
        &mut self,
        reason: EditPredictionDiscardReason,
        cx: &mut Context<Self>,
    ) -> bool {
        if reason == EditPredictionDiscardReason::Rejected {
            let completion_id = self
                .active_edit_prediction
                .as_ref()
                .and_then(|active_completion| active_completion.completion_id.clone());

            self.report_edit_prediction_event(completion_id, false, cx);
        }

        if let Some(provider) = self.edit_prediction_provider() {
            provider.discard(reason, cx);
        }

        self.take_active_edit_prediction(reason == EditPredictionDiscardReason::Ignored, cx)
    }

    pub(super) fn take_active_edit_prediction(
        &mut self,
        preserve_stale_in_menu: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(active_edit_prediction) = self.active_edit_prediction.take() else {
            if !preserve_stale_in_menu {
                self.stale_edit_prediction_in_menu = None;
            }
            return false;
        };

        self.splice_inlays(&active_edit_prediction.inlay_ids, Default::default(), cx);
        self.clear_highlights(HighlightKey::EditPredictionHighlight, cx);
        self.stale_edit_prediction_in_menu =
            preserve_stale_in_menu.then_some(active_edit_prediction);
        true
    }

    pub(super) fn update_edit_prediction_preview(
        &mut self,
        modifiers: &Modifiers,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let modifiers_held = self.edit_prediction_preview_modifiers_held(modifiers, window, cx);

        if modifiers_held {
            if matches!(
                self.edit_prediction_preview,
                EditPredictionPreview::Inactive
            ) {
                self.edit_prediction_preview = EditPredictionPreview::Active {
                    previous_scroll_position: None,
                };

                self.update_visible_edit_prediction(window, cx);
                cx.notify();
            }
        } else if let EditPredictionPreview::Active {
            previous_scroll_position,
            ..
        } = self.edit_prediction_preview
        {
            if let (Some(previous_scroll_position), Some(position_map)) =
                (previous_scroll_position, self.last_position_map.as_ref())
            {
                self.set_scroll_position(
                    previous_scroll_position
                        .scroll_position(&position_map.snapshot.display_snapshot),
                    window,
                    cx,
                );
            }

            self.edit_prediction_preview = EditPredictionPreview::Inactive;
            self.clear_row_highlights::<EditPredictionPreview>();
            self.update_visible_edit_prediction(window, cx);
            cx.notify();
        }
    }

    pub(super) fn update_visible_edit_prediction(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        if self.ime_transaction.is_some() {
            self.discard_edit_prediction(EditPredictionDiscardReason::Ignored, cx);
            return None;
        }

        let selection = self.selections.newest_anchor();
        let multibuffer = self.buffer.read(cx).snapshot(cx);
        let cursor = selection.head();
        let (cursor_text_anchor, _) = multibuffer.anchor_to_buffer_anchor(cursor)?;
        let buffer = self.buffer.read(cx).buffer(cursor_text_anchor.buffer_id)?;

        // Check project-level disable_ai setting for the current buffer
        if DisableAiSettings::is_ai_disabled_for_buffer(Some(&buffer), cx) {
            return None;
        }
        let offset_selection = selection.map(|endpoint| endpoint.to_offset(&multibuffer));

        let show_in_menu = self.show_edit_predictions_in_menu();
        let completions_menu_has_precedence = !show_in_menu
            && (self.context_menu.borrow().is_some()
                || (!self.completion_tasks.is_empty() && !self.has_active_edit_prediction()));

        if completions_menu_has_precedence
            || !offset_selection.is_empty()
            || self
                .active_edit_prediction
                .as_ref()
                .is_some_and(|completion| {
                    let Some(invalidation_range) = completion.invalidation_range.as_ref() else {
                        return false;
                    };
                    let invalidation_range = invalidation_range.to_offset(&multibuffer);
                    let invalidation_range = invalidation_range.start..=invalidation_range.end;
                    !invalidation_range.contains(&offset_selection.head())
                })
        {
            self.discard_edit_prediction(EditPredictionDiscardReason::Ignored, cx);
            return None;
        }

        self.take_active_edit_prediction(true, cx);
        let Some(provider) = self.edit_prediction_provider() else {
            self.edit_prediction_settings = EditPredictionSettings::Disabled;
            return None;
        };

        self.edit_prediction_settings =
            self.edit_prediction_settings_at_position(&buffer, cursor_text_anchor, cx);

        self.in_leading_whitespace = multibuffer.is_line_whitespace_upto(cursor);

        if self.in_leading_whitespace {
            let cursor_point = cursor.to_point(&multibuffer);
            let mut suggested_indent = None;
            multibuffer.suggested_indents_callback(
                cursor_point.row..cursor_point.row + 1,
                &mut |_, indent| {
                    suggested_indent = Some(indent);
                    ControlFlow::Break(())
                },
                cx,
            );

            if let Some(indent) = suggested_indent
                && indent.len == cursor_point.column
            {
                self.in_leading_whitespace = false;
            }
        }

        let edit_prediction = provider.suggest(&buffer, cursor_text_anchor, cx)?;

        let (completion_id, edits, predicted_cursor_position) = match edit_prediction
        {
            edit_prediction_types::EditPrediction::Local {
                id,
                edits,
                cursor_position,
                ..
            } => (id, edits, cursor_position),
            edit_prediction_types::EditPrediction::Jump {
                id,
                snapshot,
                target,
            } => {
                if let Some(provider) = &self.edit_prediction_provider {
                    provider.provider.did_show(SuggestionDisplayType::Jump, cx);
                }
                self.stale_edit_prediction_in_menu = None;
                self.active_edit_prediction = Some(EditPredictionState {
                    inlay_ids: vec![],
                    completion: EditPrediction::MoveOutside { snapshot, target },
                    completion_id: id,
                    invalidation_range: None,
                });
                cx.notify();
                return Some(());
            }
        };

        let edits = edits
            .into_iter()
            .flat_map(|(range, new_text)| {
                Some((
                    multibuffer.buffer_anchor_range_to_anchor_range(range)?,
                    new_text,
                ))
            })
            .collect::<Vec<_>>();
        if edits.is_empty() {
            return None;
        }

        let cursor_position = predicted_cursor_position.and_then(|predicted| {
            let anchor = multibuffer.anchor_in_excerpt(predicted.anchor)?;
            Some((anchor, predicted.offset))
        });

        let Some((first_edit_range, _)) = edits.first() else {
            return None;
        };
        let Some((last_edit_range, _)) = edits.last() else {
            return None;
        };

        let first_edit_start = first_edit_range.start;
        let first_edit_start_point = first_edit_start.to_point(&multibuffer);
        let edit_start_row = first_edit_start_point.row.saturating_sub(2);

        let last_edit_end = last_edit_range.end;
        let last_edit_end_point = last_edit_end.to_point(&multibuffer);
        let edit_end_row = cmp::min(multibuffer.max_point().row, last_edit_end_point.row + 2);

        let cursor_row = cursor.to_point(&multibuffer).row;

        multibuffer
            .buffer_for_id(cursor_text_anchor.buffer_id)?;

        let mut inlay_ids = Vec::new();
        let invalidation_row_range;
        let move_invalidation_row_range = if cursor_row < edit_start_row {
            Some(cursor_row..edit_end_row)
        } else if cursor_row > edit_end_row {
            Some(edit_start_row..cursor_row)
        } else {
            None
        };
        let supports_jump = self
            .edit_prediction_provider
            .as_ref()
            .map(|provider| provider.provider.supports_jump_to_edit())
            .unwrap_or(true);

        let is_move = supports_jump
            && (move_invalidation_row_range.is_some() || self.edit_predictions_hidden_for_vim_mode);
        let completion = if is_move {
            if let Some(provider) = &self.edit_prediction_provider {
                provider.provider.did_show(SuggestionDisplayType::Jump, cx);
            }
            invalidation_row_range =
                move_invalidation_row_range.unwrap_or(edit_start_row..edit_end_row);

            EditPrediction::MoveWithin {
                target: first_edit_start,
            }
        } else {
            let show_completions_in_menu = self.has_visible_completions_menu();
            let show_completions_in_buffer = !self.edit_prediction_visible_in_cursor_popover(true)
                && !self.edit_predictions_hidden_for_vim_mode;

            let display_mode = if all_edits_insertions_or_deletions(&edits, &multibuffer) {
                if provider.show_tab_accept_marker() {
                    EditDisplayMode::TabAccept
                } else {
                    EditDisplayMode::Inline
                }
            } else {
                EditDisplayMode::DiffPopover
            };

            let report_shown = match display_mode {
                EditDisplayMode::DiffPopover | EditDisplayMode::Inline => {
                    show_completions_in_buffer || show_completions_in_menu
                }
                EditDisplayMode::TabAccept => {
                    show_completions_in_menu || self.edit_prediction_preview_is_active()
                }
            };

            if report_shown && let Some(provider) = &self.edit_prediction_provider {
                let suggestion_display_type = match display_mode {
                    EditDisplayMode::DiffPopover => SuggestionDisplayType::DiffPopover,
                    EditDisplayMode::Inline | EditDisplayMode::TabAccept => {
                        SuggestionDisplayType::GhostText
                    }
                };
                provider.provider.did_show(suggestion_display_type, cx);
            }

            if show_completions_in_buffer {
                if edits
                    .iter()
                    .all(|(range, _)| range.to_offset(&multibuffer).is_empty())
                {
                    let mut inlays = Vec::new();
                    for (range, new_text) in &edits {
                        let inlay = Inlay::edit_prediction(
                            post_inc(&mut self.next_inlay_id),
                            range.start,
                            new_text.as_ref(),
                        );
                        inlay_ids.push(inlay.id);
                        inlays.push(inlay);
                    }

                    self.splice_inlays(&[], inlays, cx);
                } else {
                    let background_color = cx.theme().status().deleted_background;
                    self.highlight_text(
                        HighlightKey::EditPredictionHighlight,
                        edits.iter().map(|(range, _)| range.clone()).collect(),
                        HighlightStyle {
                            background_color: Some(background_color),
                            ..Default::default()
                        },
                        cx,
                    );
                }
            }

            invalidation_row_range = edit_start_row..edit_end_row;

            EditPrediction::Edit {
                edits,
                cursor_position,
                display_mode,
            }
        };

        let invalidation_range = multibuffer
            .anchor_before(Point::new(invalidation_row_range.start, 0))
            ..multibuffer.anchor_after(Point::new(
                invalidation_row_range.end,
                multibuffer.line_len(MultiBufferRow(invalidation_row_range.end)),
            ));

        self.stale_edit_prediction_in_menu = None;
        self.active_edit_prediction = Some(EditPredictionState {
            inlay_ids,
            completion,
            completion_id,
            invalidation_range: Some(invalidation_range),
        });

        cx.notify();

        Some(())
    }


    pub(super) fn render_edit_prediction_popover(
        &mut self,
        _text_bounds: &Bounds<Pixels>,
        _content_origin: gpui::Point<Pixels>,
        _right_margin: Pixels,
        _editor_snapshot: &EditorSnapshot,
        _visible_row_range: Range<DisplayRow>,
        _scroll_top: ScrollOffset,
        _scroll_bottom: ScrollOffset,
        _line_layouts: &[LineWithInvisibles],
        _line_height: Pixels,
        _scroll_position: gpui::Point<ScrollOffset>,
        _scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
        _newest_selection_head: Option<DisplayPoint>,
        _editor_width: Pixels,
        _style: &EditorStyle,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<(AnyElement, gpui::Point<Pixels>)> {
        None
    }

    pub(super) fn edit_prediction_cursor_popover_height(&self) -> Pixels {
        px(30.)
    }

    pub(super) fn render_edit_prediction_cursor_popover(
        &self,
        _min_width: Pixels,
        _max_width: Pixels,
        _cursor_point: Point,
        _style: &EditorStyle,
        _window: &mut Window,
        _cx: &mut Context<Editor>,
    ) -> Option<AnyElement> {
        None
    }

    fn edit_prediction_preview_modifiers_held(
        &self,
        modifiers: &Modifiers,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let can_supersede_active_menu =
            self.context_menu.borrow().as_ref().is_none_or(|menu| {
                !menu.visible() || matches!(menu, CodeContextMenu::Completions(_))
            });

        if !can_supersede_active_menu {
            return false;
        }

        let key_context = self.key_context_internal(true, window, cx);
        let actions: [&dyn Action; 3] = [
            &AcceptEditPrediction,
            &AcceptNextWordEditPrediction,
            &AcceptNextLineEditPrediction,
        ];

        actions.into_iter().any(|action| {
            window
                .bindings_for_action_in_context(action, key_context.clone())
                .into_iter()
                .rev()
                .any(|binding| {
                    binding.keystrokes().first().is_some_and(|keystroke| {
                        keystroke.modifiers().modified() && keystroke.modifiers() == modifiers
                    })
                })
        })
    }

    fn edit_predictions_disabled_in_scope(
        &self,
        buffer: &Entity<Buffer>,
        buffer_position: language::Anchor,
        cx: &App,
    ) -> bool {
        let snapshot = buffer.read(cx).snapshot();
        let settings = snapshot.settings_at(buffer_position, cx);

        let Some(scope) = snapshot.language_scope_at(buffer_position) else {
            return false;
        };

        scope.override_name().is_some_and(|scope_name| {
            settings
                .edit_predictions_disabled_in
                .iter()
                .any(|s| s == scope_name)
        })
    }

    fn edit_prediction_settings_at_position(
        &self,
        buffer: &Entity<Buffer>,
        buffer_position: language::Anchor,
        cx: &App,
    ) -> EditPredictionSettings {
        if !self.mode.is_full()
            || !self.show_edit_predictions_override.unwrap_or(true)
            || self.edit_predictions_disabled_in_scope(buffer, buffer_position, cx)
        {
            return EditPredictionSettings::Disabled;
        }

        if !LanguageSettings::for_buffer(&buffer.read(cx), cx).show_edit_predictions {
            return EditPredictionSettings::Disabled;
        };

        let by_provider = matches!(
            self.menu_edit_predictions_policy,
            MenuEditPredictionsPolicy::ByProvider
        );

        let show_in_menu = by_provider
            && self
                .edit_prediction_provider
                .as_ref()
                .is_some_and(|provider| provider.provider.show_predictions_in_menu());

        let file = buffer.read(cx).file();
        let preview_requires_modifier =
            all_language_settings(file, cx).edit_predictions_mode() == EditPredictionsMode::Subtle;

        EditPredictionSettings::Enabled {
            show_in_menu,
            preview_requires_modifier,
        }
    }

    fn should_show_edit_predictions(&self) -> bool {
        self.snippet_stack.is_empty() && self.edit_predictions_enabled()
    }

    fn edit_predictions_enabled_in_buffer(
        &self,
        buffer: &Entity<Buffer>,
        buffer_position: language::Anchor,
        cx: &App,
    ) -> bool {
        maybe!({
            if self.read_only(cx) || self.leader_id.is_some() {
                return Some(false);
            }
            let provider = self.edit_prediction_provider()?;
            if !provider.is_enabled(buffer, buffer_position, cx) {
                return Some(false);
            }
            let buffer = buffer.read(cx);
            let Some(file) = buffer.file() else {
                return Some(true);
            };
            let settings = all_language_settings(Some(file), cx);
            Some(settings.edit_predictions_enabled_for_file(file, cx))
        })
        .unwrap_or(false)
    }

    fn report_edit_prediction_event(&self, id: Option<SharedString>, accepted: bool, cx: &App) {
        let Some(provider) = self.edit_prediction_provider() else {
            return;
        };

        let buffer_snapshot = self.buffer.read(cx).snapshot(cx);
        let Some((position, _)) =
            buffer_snapshot.anchor_to_buffer_anchor(self.selections.newest_anchor().head())
        else {
            return;
        };
        let Some(buffer) = self.buffer.read(cx).buffer(position.buffer_id) else {
            return;
        };

        let extension = buffer
            .read(cx)
            .file()
            .and_then(|file| Some(file.path().extension()?.to_string()));

        let event_type = match accepted {
            true => "Edit Prediction Accepted",
            false => "Edit Prediction Discarded",
        };
        telemetry::event!(
            event_type,
            provider = provider.name(),
            prediction_id = id,
            suggestion_accepted = accepted,
            file_extension = extension,
        );
    }

    fn open_editor_at_anchor(
        snapshot: &language::BufferSnapshot,
        target: language::Anchor,
        workspace: &Entity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        workspace.update(cx, |workspace, cx| {
            let path = snapshot.file().map(|file| file.full_path(cx));
            let Some(path) =
                path.and_then(|path| workspace.project().read(cx).find_project_path(path, cx))
            else {
                return Task::ready(Err(anyhow::anyhow!("Project path not found")));
            };
            let target = text::ToPoint::to_point(&target, snapshot);
            let item = workspace.open_path(path, None, true, window, cx);
            window.spawn(cx, async move |cx| {
                let Some(editor) = item.await?.downcast::<Editor>() else {
                    return Ok(());
                };
                editor
                    .update_in(cx, |editor, window, cx| {
                        editor.go_to_singleton_buffer_point(target, window, cx);
                    })
                    .ok();
                anyhow::Ok(())
            })
        })
    }

}


#[cfg(test)]
impl Editor {
    pub(super) fn set_menu_edit_predictions_policy(&mut self, value: MenuEditPredictionsPolicy) {
        self.menu_edit_predictions_policy = value;
    }
}

fn all_edits_insertions_or_deletions(
    edits: &Vec<(Range<Anchor>, Arc<str>)>,
    snapshot: &MultiBufferSnapshot,
) -> bool {
    let mut all_insertions = true;
    let mut all_deletions = true;

    for (range, new_text) in edits.iter() {
        let range_is_empty = range.to_offset(snapshot).is_empty();
        let text_is_empty = new_text.is_empty();

        if range_is_empty != text_is_empty {
            if range_is_empty {
                all_deletions = false;
            } else {
                all_insertions = false;
            }
        } else {
            return false;
        }

        if !all_insertions && !all_deletions {
            return false;
        }
    }
    all_insertions || all_deletions
}
