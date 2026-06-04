use crate::{
    Anchor, Editor, EditorSnapshot, GotoDefinitionKind, HighlightKey, PointForPosition,
    scroll::ScrollAmount,
};
use gpui::{App, AsyncWindowContext, Context, Entity, Modifiers, Pixels, Task, Window};
use language::ToOffset;
use linkify::{LinkFinder, LinkKind};
use lsp::LanguageServerId;
use project::{InlayId, LocationLink, Project, ResolvedPath};
use regex::Regex;
use std::{ops::Range, str::FromStr as _, sync::LazyLock};
use text::OffsetRangeExt;
use util::{ResultExt, paths::PathWithPosition};

#[derive(Debug)]
pub struct HoveredLinkState {
    pub last_trigger_point: TriggerPoint,
    pub preferred_kind: GotoDefinitionKind,
    pub symbol_range: Option<RangeInEditor>,
    pub links: Vec<HoverLink>,
    pub task: Option<Task<Option<()>>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum RangeInEditor {
    Text(Range<Anchor>),
    Inlay(InlayHighlight),
}

impl RangeInEditor {
    pub fn as_text_range(&self) -> Option<Range<Anchor>> {
        match self {
            Self::Text(range) => Some(range.clone()),
            Self::Inlay(_) => None,
        }
    }

    pub fn point_within_range(
        &self,
        trigger_point: &TriggerPoint,
        snapshot: &EditorSnapshot,
    ) -> bool {
        match (self, trigger_point) {
            (Self::Text(range), TriggerPoint::Text(point)) => {
                let point_after_start = range.start.cmp(point, &snapshot.buffer_snapshot()).is_le();
                point_after_start && range.end.cmp(point, &snapshot.buffer_snapshot()).is_ge()
            }
            (Self::Inlay(highlight), TriggerPoint::InlayHint(point, _, _)) => {
                highlight.inlay == point.inlay
                    && highlight.range.contains(&point.range.start)
                    && highlight.range.contains(&point.range.end)
            }
            (Self::Inlay(_), TriggerPoint::Text(_))
            | (Self::Text(_), TriggerPoint::InlayHint(_, _, _)) => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum HoverLink {
    Url(String),
    File(ResolvedFileTarget),
    Text(LocationLink),
    LspLocation(lsp::Location, LanguageServerId),
}

pub fn document_link_target_to_hover_link(target: &str, server_id: LanguageServerId) -> HoverLink {
    if let Ok(url) = url::Url::parse(target)
        && url.scheme() == "file"
        && let Ok(uri) = lsp::Uri::from_str(target)
    {
        let position = url
            .fragment()
            .and_then(parse_uri_fragment_position)
            .unwrap_or_default();
        return HoverLink::LspLocation(
            lsp::Location {
                uri,
                range: lsp::Range::new(position, position),
            },
            server_id,
        );
    }
    HoverLink::Url(target.to_string())
}

fn parse_uri_fragment_position(fragment: &str) -> Option<lsp::Position> {
    let stripped = fragment.strip_prefix('L').unwrap_or(fragment);
    let (line_str, column_str) = match stripped.split_once([',', ':']) {
        Some((line, column)) => (line, Some(column)),
        None => (stripped, None),
    };
    let line = line_str.parse::<u32>().ok()?.checked_sub(1)?;
    let character = column_str
        .and_then(|column| column.parse::<u32>().ok())
        .and_then(|column| column.checked_sub(1))
        .unwrap_or(0);
    Some(lsp::Position { line, character })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlayHighlight {
    pub inlay: InlayId,
    pub inlay_position: Anchor,
    pub range: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TriggerPoint {
    Text(Anchor),
    InlayHint(InlayHighlight, lsp::Location, LanguageServerId),
}

impl TriggerPoint {
    fn anchor(&self) -> &Anchor {
        match self {
            TriggerPoint::Text(anchor) => anchor,
            TriggerPoint::InlayHint(inlay_range, _, _) => &inlay_range.inlay_position,
        }
    }
}

pub fn exclude_link_to_position(
    buffer: &Entity<language::Buffer>,
    current_position: &text::Anchor,
    location: &LocationLink,
    cx: &App,
) -> bool {
    let snapshot = buffer.read(cx).snapshot();
    !(buffer == &location.target.buffer
        && current_position
            .bias_right(&snapshot)
            .cmp(&location.target.range.start, &snapshot)
            .is_ge()
        && current_position
            .cmp(&location.target.range.end, &snapshot)
            .is_le())
}

impl Editor {
    pub(crate) fn update_hovered_link(
        &mut self,
        _point_for_position: PointForPosition,
        _mouse_position: Option<gpui::Point<Pixels>>,
        _snapshot: &EditorSnapshot,
        _modifiers: Modifiers,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.hide_hovered_link(cx);
    }

    pub(crate) fn hide_hovered_link(&mut self, cx: &mut Context<Self>) {
        self.hovered_link_state.take();
        self.clear_highlights(HighlightKey::HoveredLinkState, cx);
    }

    pub(crate) fn handle_click_hovered_link(
        &mut self,
        _point: PointForPosition,
        _modifiers: Modifiers,
        _window: &mut Window,
        _cx: &mut Context<Editor>,
    ) {
    }

    pub fn scroll_hover(
        &mut self,
        _amount: ScrollAmount,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> bool {
        false
    }
}

pub fn show_link_definition(
    _shift_held: bool,
    _editor: &mut Editor,
    _trigger_point: TriggerPoint,
    _snapshot: &EditorSnapshot,
    _window: &mut Window,
    _cx: &mut Context<Editor>,
) {
}

pub(crate) fn find_url(
    buffer: &Entity<language::Buffer>,
    position: text::Anchor,
    cx: &AsyncWindowContext,
) -> Option<(Range<text::Anchor>, String)> {
    const LIMIT: usize = 2048;

    let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());

    let offset = position.to_offset(&snapshot);
    let mut token_start = offset;
    let mut token_end = offset;
    let mut found_start = false;
    let mut found_end = false;

    for ch in snapshot.reversed_chars_at(offset).take(LIMIT) {
        if ch.is_whitespace() {
            found_start = true;
            break;
        }
        token_start -= ch.len_utf8();
    }
    if !found_start && token_start != 0 {
        return None;
    }

    for ch in snapshot
        .chars_at(offset)
        .take(LIMIT - (offset - token_start))
    {
        if ch.is_whitespace() {
            found_end = true;
            break;
        }
        token_end += ch.len_utf8();
    }
    if !found_end && (token_end - token_start >= LIMIT) {
        return None;
    }

    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Url]);
    let input = snapshot
        .text_for_range(token_start..token_end)
        .collect::<String>();

    let relative_offset = offset - token_start;
    for link in finder.links(&input) {
        if link.start() <= relative_offset && link.end() >= relative_offset {
            let range = snapshot.anchor_before(token_start + link.start())
                ..snapshot.anchor_after(token_start + link.end());
            return Some((range, link.as_str().to_string()));
        }
    }
    None
}

pub(crate) fn find_url_from_range(
    buffer: &Entity<language::Buffer>,
    range: Range<text::Anchor>,
    cx: &AsyncWindowContext,
) -> Option<String> {
    const LIMIT: usize = 2048;

    let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());

    let start_offset = range.start.to_offset(&snapshot);
    let end_offset = range.end.to_offset(&snapshot);

    let mut token_start = start_offset.min(end_offset);
    let mut token_end = start_offset.max(end_offset);

    let range_len = token_end - token_start;

    if range_len >= LIMIT {
        return None;
    }

    for ch in snapshot.chars_at(token_start).take(range_len) {
        if !ch.is_whitespace() {
            break;
        }
        token_start += ch.len_utf8();
    }

    for ch in snapshot.reversed_chars_at(token_end).take(range_len) {
        if !ch.is_whitespace() {
            break;
        }
        token_end -= ch.len_utf8();
    }

    if token_start >= token_end {
        return None;
    }

    let text = snapshot
        .text_for_range(token_start..token_end)
        .collect::<String>();

    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Url]);

    if let Some(link) = finder.links(&text).next()
        && link.start() == 0
        && link.end() == text.len()
    {
        return Some(link.as_str().to_string());
    }

    None
}

#[derive(Debug, Clone)]
pub struct ResolvedFileTarget {
    pub resolved_path: ResolvedPath,
    pub row: Option<u32>,
    pub column: Option<u32>,
}

impl ResolvedFileTarget {
    pub fn navigate_item_to_position(
        &self,
        item: Box<dyn crate::ItemHandle>,
        cx: &mut AsyncWindowContext,
    ) {
        if let Some(row) = self.row {
            let col = self.column.unwrap_or(0);
            if let Some(active_editor) = item.downcast::<crate::Editor>() {
                active_editor
                    .downgrade()
                    .update_in(cx, |editor, window, cx| {
                        let row = row.saturating_sub(1);
                        let col = col.saturating_sub(1);
                        let Some(buffer) = editor.buffer().read(cx).as_singleton() else {
                            return;
                        };
                        let point = buffer
                            .read(cx)
                            .snapshot()
                            .point_from_external_input(row, col);
                        editor.go_to_singleton_buffer_point_silently(point, window, cx);
                    })
                    .log_err();
            }
        }
    }
}

pub(crate) async fn find_file(
    buffer: &Entity<language::Buffer>,
    project: Option<Entity<Project>>,
    position: text::Anchor,
    cx: &mut AsyncWindowContext,
) -> Option<(Range<text::Anchor>, ResolvedFileTarget)> {
    let project = project?;
    let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());
    let scope = snapshot.language_scope_at(position);
    let (range, candidate_file_path) = surrounding_filename(&snapshot, position)?;
    let candidate_len = candidate_file_path.len();

    async fn check_path(
        candidate_file_path: &str,
        project: &Entity<Project>,
        buffer: &Entity<language::Buffer>,
        cx: &mut AsyncWindowContext,
    ) -> Option<ResolvedPath> {
        project
            .update(cx, |project, cx| {
                project.resolve_path_in_buffer(candidate_file_path, buffer, cx)
            })
            .await
            .filter(|s| s.is_file())
    }

    let pattern_candidates = link_pattern_file_candidates(&candidate_file_path);

    let make_range = |pattern_range: &Range<usize>| -> Range<text::Anchor> {
        let offset_range = range.to_offset(&snapshot);
        let actual_start = offset_range.start + pattern_range.start;
        let actual_end = offset_range.end - (candidate_len - pattern_range.end);
        snapshot.anchor_before(actual_start)..snapshot.anchor_after(actual_end)
    };

    for (pattern_candidate, pattern_range) in &pattern_candidates {
        if let Some(existing_path) = check_path(&pattern_candidate, &project, buffer, cx).await {
            return Some((
                make_range(pattern_range),
                ResolvedFileTarget {
                    resolved_path: existing_path,
                    row: None,
                    column: None,
                },
            ));
        }

        let parsed = PathWithPosition::parse_str(pattern_candidate);
        let parsed_path = parsed.path.to_string_lossy();

        if parsed.row.is_some() {
            if let Some(existing_path) = check_path(&parsed_path, &project, buffer, cx).await {
                return Some((
                    make_range(pattern_range),
                    ResolvedFileTarget {
                        resolved_path: existing_path,
                        row: parsed.row,
                        column: parsed.column,
                    },
                ));
            }
        }

        if let Some(scope) = &scope {
            for suffix in scope.path_suffixes() {
                if pattern_candidate.ends_with(format!(".{suffix}").as_str()) {
                    continue;
                }

                let suffixed_candidate = format!("{pattern_candidate}.{suffix}");
                if let Some(existing_path) =
                    check_path(&suffixed_candidate, &project, buffer, cx).await
                {
                    return Some((
                        make_range(pattern_range),
                        ResolvedFileTarget {
                            resolved_path: existing_path,
                            row: None,
                            column: None,
                        },
                    ));
                }
            }

            if parsed.row.is_some() {
                for suffix in scope.path_suffixes() {
                    if parsed_path.ends_with(&format!(".{suffix}")) {
                        continue;
                    }

                    let suffixed_candidate = format!("{parsed_path}.{suffix}");
                    if let Some(existing_path) =
                        check_path(&suffixed_candidate, &project, buffer, cx).await
                    {
                        return Some((
                            make_range(pattern_range),
                            ResolvedFileTarget {
                                resolved_path: existing_path,
                                row: parsed.row,
                                column: parsed.column,
                            },
                        ));
                    }
                }
            }
        }
    }
    None
}

fn link_pattern_file_candidates(candidate: &str) -> Vec<(String, Range<usize>)> {
    static MD_LINK_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"]\(([^)]*)\)").expect("Failed to create REGEX"));

    const LEADING_PUNCTUATION: &[char] = &['`', '(', '[', '{', '<', '"', '\''];
    const TRAILING_PUNCTUATION: &[char] = &[
        '`', ')', ']', '}', '>', '"', '\'', '.', ',', ':', ';', '!', '?',
    ];

    let candidate_len = candidate.len();
    let mut candidates = Vec::new();

    let mut start = 0;
    let mut end = candidate_len;

    for ch in candidate.chars() {
        if LEADING_PUNCTUATION.contains(&ch) {
            start += ch.len_utf8();
        } else {
            break;
        }
    }

    for ch in candidate.chars().rev() {
        if TRAILING_PUNCTUATION.contains(&ch) {
            end -= ch.len_utf8();
        } else {
            break;
        }
    }

    if start < end && (start > 0 || end < candidate_len) {
        candidates.push((candidate[start..end].to_string(), start..end));
    }

    if let Some(captures) = MD_LINK_REGEX.captures(candidate) {
        if let Some(link) = captures.get(1) {
            let link_str = link.as_str().to_string();
            let link_range = link.range();
            if !candidates.iter().any(|(s, _)| s == &link_str) {
                candidates.push((link_str, link_range));
            }
        }
    }

    candidates.push((candidate.to_string(), 0..candidate_len));

    candidates
}

fn surrounding_filename(
    snapshot: &language::BufferSnapshot,
    position: text::Anchor,
) -> Option<(Range<text::Anchor>, String)> {
    const LIMIT: usize = 2048;

    let offset = position.to_offset(&snapshot);
    let mut token_start = offset;
    let mut token_end = offset;
    let mut found_start = false;
    let mut found_end = false;
    let mut inside_quotes = false;

    let mut filename = String::new();

    let mut backwards = snapshot.reversed_chars_at(offset).take(LIMIT).peekable();
    while let Some(ch) = backwards.next() {
        if ch.is_whitespace() && backwards.peek() == Some(&'\\') {
            filename.push(ch);
            token_start -= ch.len_utf8();
            backwards.next();
            token_start -= '\\'.len_utf8();
            continue;
        }
        if ch.is_whitespace() {
            found_start = true;
            break;
        }
        if (ch == '"' || ch == '\'') && !inside_quotes {
            found_start = true;
            inside_quotes = true;
            break;
        }

        filename.push(ch);
        token_start -= ch.len_utf8();
    }
    if !found_start && token_start != 0 {
        return None;
    }

    filename = filename.chars().rev().collect();

    let mut forwards = snapshot
        .chars_at(offset)
        .take(LIMIT - (offset - token_start))
        .peekable();
    while let Some(ch) = forwards.next() {
        if ch == '\\' && forwards.peek().is_some_and(|ch| ch.is_whitespace()) {
            token_end += ch.len_utf8();
            let whitespace = forwards.next().unwrap();
            token_end += whitespace.len_utf8();
            filename.push(whitespace);
            continue;
        }

        if ch.is_whitespace() {
            found_end = true;
            break;
        }
        if ch == '"' || ch == '\'' {
            if inside_quotes {
                found_end = true;
                break;
            } else {
                inside_quotes = true;
                token_end += ch.len_utf8();
                continue;
            }
        }
        filename.push(ch);
        token_end += ch.len_utf8();
    }

    if !found_end && (token_end - token_start >= LIMIT) {
        return None;
    }

    if filename.is_empty() {
        return None;
    }

    let range = snapshot.anchor_before(token_start)..snapshot.anchor_after(token_end);

    Some((range, filename))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_uri_fragment_position() {
        assert_eq!(
            parse_uri_fragment_position("9,16"),
            Some(lsp::Position {
                line: 8,
                character: 15,
            })
        );
        assert_eq!(
            parse_uri_fragment_position("33,33"),
            Some(lsp::Position {
                line: 32,
                character: 32,
            })
        );
        assert_eq!(
            parse_uri_fragment_position("L42"),
            Some(lsp::Position {
                line: 41,
                character: 0,
            })
        );
        assert_eq!(
            parse_uri_fragment_position("L9:16"),
            Some(lsp::Position {
                line: 8,
                character: 15,
            })
        );
        assert_eq!(parse_uri_fragment_position("0,0"), None);
        assert_eq!(parse_uri_fragment_position("L0"), None);
        assert_eq!(parse_uri_fragment_position("not_a_position"), None);
    }
}
