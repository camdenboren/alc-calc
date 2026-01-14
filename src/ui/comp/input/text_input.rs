// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

// Adapted from: https://github.com/zed-industries/zed/blob/main/crates/gpui/examples/input.rs

use crate::ui::{
    ActiveCtrl,
    comp::input::{cursor_state::CursorState, text_element::TextElement},
    util::theme::ActiveTheme,
};
use gpui::{
    App, Bounds, ClipboardItem, Context, CursorStyle, Entity, EntityInputHandler, EventEmitter,
    FocusHandle, Focusable, KeyBinding, KeyDownEvent, MouseButton, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, Pixels, Point, ShapedLine, SharedString, Subscription, UTF16Selection, Window,
    actions, div, point, prelude::*, px,
};
use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;

actions!(
    text_input,
    [
        Backspace,
        Delete,
        Left,
        Right,
        SelectLeft,
        SelectRight,
        SelectAll,
        Home,
        End,
        ShowCharacterPalette,
        Paste,
        Cut,
        Copy,
    ]
);

enum InputEvent {
    Focus,
    Blur,
}

const CONTEXT: &str = "TextInput";

pub struct TextInput {
    pub cursor_state: Entity<CursorState>,
    pub focus_handle: FocusHandle,
    pub content: SharedString,
    pub placeholder: SharedString,
    pub selected_range: Range<usize>,
    selected_word_range: Range<usize>,
    selection_reversed: bool,
    pub marked_range: Option<Range<usize>>,
    pub last_layout: Option<ShapedLine>,
    pub last_bounds: Option<Bounds<Pixels>>,
    is_selecting: bool,
    _subscriptions: Vec<Subscription>,
}

impl TextInput {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        placeholder: SharedString,
        tab_index: isize,
    ) -> Self {
        let ctrl = cx.ctrl();
        cx.bind_keys([
            KeyBinding::new("backspace", Backspace, Some(CONTEXT)),
            KeyBinding::new("delete", Delete, Some(CONTEXT)),
            KeyBinding::new("left", Left, Some(CONTEXT)),
            KeyBinding::new("right", Right, Some(CONTEXT)),
            KeyBinding::new("shift-left", SelectLeft, Some(CONTEXT)),
            KeyBinding::new("shift-right", SelectRight, Some(CONTEXT)),
            KeyBinding::new(&format!("{ctrl}-a"), SelectAll, Some(CONTEXT)),
            KeyBinding::new(&format!("{ctrl}-v"), Paste, Some(CONTEXT)),
            KeyBinding::new(&format!("{ctrl}-c"), Copy, Some(CONTEXT)),
            KeyBinding::new(&format!("{ctrl}-x"), Cut, Some(CONTEXT)),
            KeyBinding::new("home", Home, Some(CONTEXT)),
            KeyBinding::new("end", End, Some(CONTEXT)),
            KeyBinding::new(
                &format!("{ctrl}-shift-space"),
                ShowCharacterPalette,
                Some(CONTEXT),
            ),
        ]);

        let focus_handle = cx.focus_handle().tab_index(tab_index).tab_stop(true);
        cx.on_focus(&focus_handle, window, Self::on_focus).detach();
        cx.on_blur(&focus_handle, window, Self::on_blur).detach();
        let cursor_state = cx.new(|_| CursorState::default());

        Self {
            cursor_state: cursor_state.clone(),
            focus_handle,
            content: "".into(),
            placeholder,
            selected_range: 0..0,
            selected_word_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            is_selecting: false,
            _subscriptions: vec![
                cx.observe(&cursor_state, |_, _, cx| cx.notify()),
                cx.observe_window_activation(window, |input, window, cx| {
                    if window.is_window_active() {
                        let active = window.is_window_active();
                        input.cursor_state.update(cx, |blink_manager, cx| {
                            if active {
                                blink_manager.enable(cx);
                            } else {
                                blink_manager.disable(cx);
                            }
                        });
                    }
                }),
            ],
        }
    }

    pub fn focus(&self, window: &mut Window) {
        self.focus_handle.focus(window)
    }

    pub fn is_focused(&self, window: &mut Window) -> bool {
        self.focus_handle.is_focused(window)
    }

    fn on_focus(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.cursor_state.update(cx, |cursor, cx| {
            cursor.enable(cx);
        });
        cx.emit(InputEvent::Focus);
    }

    fn on_blur(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.cursor_state.update(cx, |cursor, cx| {
            cursor.disable(cx);
        });
        if !self.selected_range.is_empty() {
            self.move_to(0, cx);
        }
        cx.emit(InputEvent::Blur);
    }

    fn pause_blink(&mut self, cx: &mut Context<Self>) {
        self.cursor_state.update(cx, |cursor, cx| {
            cursor.pause_blinking(cx);
        });
    }

    pub fn show_cursor(&self, cx: &mut Context<Self>) {
        self.cursor_state
            .update(cx, |cursor, cx| cursor.show_cursor(cx));
    }

    pub fn should_show_cursor(&self, window: &mut Window, cx: &App) -> bool {
        self.is_focused(window) && self.cursor_state.read(cx).visible()
    }

    fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        self.pause_blink(cx);
        if self.selected_range.is_empty() {
            self.move_to(self.previous_boundary(self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.start, cx)
        }
    }

    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        self.pause_blink(cx);
        if self.selected_range.is_empty() {
            self.move_to(self.next_boundary(self.selected_range.end), cx);
        } else {
            self.move_to(self.selected_range.end, cx)
        }
    }

    fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.previous_boundary(self.cursor_offset()), cx);
    }

    fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.next_boundary(self.cursor_offset()), cx);
    }

    fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
        self.select_to(self.content.len(), cx)
    }

    fn select_word(&mut self, offset: usize, window: &mut Window, cx: &mut Context<Self>) {
        let mut start = self.offset_to_utf16(offset);
        let mut end = start;
        let prev_text = self
            .text_for_range(0..start, &mut None, window, cx)
            .unwrap_or_default();
        let next_text = self
            .text_for_range(end..self.content.len(), &mut None, window, cx)
            .unwrap_or_default();

        let prev_chars = prev_text.chars().rev().peekable();
        let next_chars = next_text.chars().peekable();

        for c in prev_chars {
            if c.is_whitespace() {
                break;
            }

            start -= c.len_utf16();
        }

        for c in next_chars {
            if c.is_whitespace() {
                break;
            }

            end += c.len_utf16();
        }

        self.selected_range = self.range_from_utf16(&(start..end));
        self.selected_word_range = self.selected_range.clone();
        cx.notify()
    }

    fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) {
        self.pause_blink(cx);
        self.move_to(0, cx);
    }

    fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) {
        self.pause_blink(cx);
        self.move_to(self.content.len(), cx);
    }

    fn backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.previous_boundary(self.cursor_offset()), cx)
        }
        self.replace_text_in_range(None, "", window, cx);
        self.pause_blink(cx);
    }

    fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.next_boundary(self.cursor_offset()), cx)
        }
        self.replace_text_in_range(None, "", window, cx);
        self.pause_blink(cx);
    }

    fn on_key_down(&mut self, _: &KeyDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.pause_blink(cx);
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.is_selecting = true;

        let offset = self.index_for_mouse_position(event.position);
        if event.button == MouseButton::Left && event.click_count == 2 {
            self.select_word(offset, window, cx);
            return;
        }

        if event.button == MouseButton::Left && event.click_count == 3 {
            self.select_all(&SelectAll, window, cx);
            return;
        }

        if event.modifiers.shift {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        } else {
            self.move_to(self.index_for_mouse_position(event.position), cx)
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, _: &mut Context<Self>) {
        self.is_selecting = false;
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.is_selecting {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        }
    }

    fn show_character_palette(
        &mut self,
        _: &ShowCharacterPalette,
        window: &mut Window,
        _: &mut Context<Self>,
    ) {
        window.show_character_palette();
    }

    fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            self.replace_text_in_range(None, &text.replace("\n", " "), window, cx);
        }
    }

    fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content
                    .get(self.selected_range.clone())
                    .unwrap_or(&self.content[0..])
                    .to_string(),
            ));
        }
    }

    fn cut(&mut self, _: &Cut, window: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content
                    .get(self.selected_range.clone())
                    .unwrap_or(&self.content[0..])
                    .to_string(),
            ));
            self.replace_text_in_range(None, "", window, cx)
        }
    }

    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        self.selected_range = offset..offset;
        self.pause_blink(cx);

        cx.notify()
    }

    pub fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    fn index_for_mouse_position(&self, position: Point<Pixels>) -> usize {
        if self.content.is_empty() {
            return 0;
        }

        let (Some(bounds), Some(line)) = (self.last_bounds.as_ref(), self.last_layout.as_ref())
        else {
            return 0;
        };
        if position.y < bounds.top() {
            return 0;
        }
        if position.y > bounds.bottom() {
            return self.content.len();
        }
        line.closest_index_for_x(position.x - bounds.left())
    }

    fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        if self.selection_reversed {
            self.selected_range.start = offset
        } else {
            self.selected_range.end = offset
        };
        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }
        cx.notify()
    }

    fn offset_from_utf16(&self, offset: usize) -> usize {
        let mut utf8_offset = 0;
        let mut utf16_count = 0;

        for ch in self.content.chars() {
            if utf16_count >= offset {
                break;
            }
            utf16_count += ch.len_utf16();
            utf8_offset += ch.len_utf8();
        }

        utf8_offset
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut utf8_count = 0;

        for ch in self.content.chars() {
            if utf8_count >= offset {
                break;
            }
            utf8_count += ch.len_utf8();
            utf16_offset += ch.len_utf16();
        }

        utf16_offset
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    fn range_from_utf16(&self, range_utf16: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range_utf16.start)..self.offset_from_utf16(range_utf16.end)
    }

    fn previous_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .rev()
            .find_map(|(idx, _)| (idx < offset).then_some(idx))
            .unwrap_or(0)
    }

    fn next_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .find_map(|(idx, _)| (idx > offset).then_some(idx))
            .unwrap_or(self.content.len())
    }
}

impl EventEmitter<InputEvent> for TextInput {}

impl EntityInputHandler for TextInput {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        actual_range.replace(self.range_to_utf16(&range));
        Some(
            self.content
                .get(range)
                .unwrap_or(&self.content[0..])
                .to_string(),
        )
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.range_to_utf16(&self.selected_range),
            reversed: self.selection_reversed,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|range| self.range_to_utf16(range))
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.marked_range = None;
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        self.content = (self
            .content
            .get(0..range.start)
            .unwrap_or(&self.content[0..])
            .to_owned()
            + new_text
            + self.content.get(range.end..).unwrap_or(&self.content[0..]))
        .into();
        self.selected_range = range.start + new_text.len()..range.start + new_text.len();
        self.marked_range.take();
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        self.content = (self
            .content
            .get(0..range.start)
            .unwrap_or(&self.content[0..])
            .to_owned()
            + new_text
            + self.content.get(range.end..).unwrap_or(&self.content[0..]))
        .into();
        self.marked_range = Some(range.start..range.start + new_text.len());
        self.selected_range = new_selected_range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .map(|new_range| new_range.start + range.start..new_range.end + range.end)
            .unwrap_or_else(|| range.start + new_text.len()..range.start + new_text.len());

        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let last_layout = self.last_layout.as_ref()?;
        let range = self.range_from_utf16(&range_utf16);
        Some(Bounds::from_corners(
            point(
                bounds.left() + last_layout.x_for_index(range.start),
                bounds.top(),
            ),
            point(
                bounds.left() + last_layout.x_for_index(range.end),
                bounds.bottom(),
            ),
        ))
    }

    fn character_index_for_point(
        &mut self,
        point: gpui::Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let line_point = self.last_bounds?.localize(&point)?;
        let last_layout = self.last_layout.as_ref()?;

        assert_eq!(last_layout.text, self.content);
        let utf8_index = last_layout.index_for_x(point.x - line_point.x)?;
        Some(self.offset_to_utf16(utf8_index))
    }
}

impl Render for TextInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .key_context(CONTEXT)
            .track_focus(&self.focus_handle(cx))
            .cursor(CursorStyle::IBeam)
            .on_action(cx.listener(Self::backspace))
            .on_action(cx.listener(Self::delete))
            .on_action(cx.listener(Self::left))
            .on_action(cx.listener(Self::right))
            .on_action(cx.listener(Self::select_left))
            .on_action(cx.listener(Self::select_right))
            .on_action(cx.listener(Self::select_all))
            .on_action(cx.listener(Self::home))
            .on_action(cx.listener(Self::end))
            .on_action(cx.listener(Self::show_character_palette))
            .on_action(cx.listener(Self::paste))
            .on_action(cx.listener(Self::cut))
            .on_action(cx.listener(Self::copy))
            .on_key_down(cx.listener(Self::on_key_down))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_move(cx.listener(Self::on_mouse_move))
            .line_height(px(30.))
            .child(
                div()
                    .h(px(30. + 4. * 2.))
                    .w(px(120. + 4. * 2.))
                    .p(px(4.))
                    .bg(cx.theme().background)
                    .rounded_md()
                    .child(TextElement { input: cx.entity() }),
            )
    }
}

impl Focusable for TextInput {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
