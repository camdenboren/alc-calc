// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

// Adapted from: https://github.com/zed-industries/zed/blob/main/crates/gpui/examples/input.rs

use crate::ui::{
    comp::{
        input::text_input::TextInput,
        toast::{ToastVariant, toast},
    },
    util::theme::ActiveTheme,
};
use gpui::{
    App, Bounds, ElementId, ElementInputHandler, Entity, GlobalElementId, LayoutId, PaintQuad,
    Pixels, ShapedLine, Style, TextRun, UnderlineStyle, Window, fill, point, prelude::*, px,
    relative, size,
};

const MAX_DIGITS: usize = 9;

/// An `Element` implementation for `TextInput`, enabling lower-level layout requests,
/// bounds calculations, and element painting via Taffy (the layout library used by GPUI)
///
/// Note that `TextElement` is the parent of `TextInput` in terms of _entity ownership_,
/// but `TextInput` is the parent in terms of _rendering_
pub struct TextElement {
    pub input: Entity<TextInput>,
}

pub struct PrepaintState {
    line: Option<ShapedLine>,
    cursor: Option<PaintQuad>,
    selection: Option<PaintQuad>,
}

impl IntoElement for TextElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for TextElement {
    type RequestLayoutState = ();

    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = window.line_height().into();
        (window.request_layout(style, [], cx), ())
    }

    /// Construct the `PrepaintState` used to paint the `TextInput` based on the raw text
    /// content and cursor + selection states of the associated `TextInput` `Entity`
    ///
    /// This implementation also ensures that a maximum of `MAX_DIGITS` are displayed in
    /// any `TextInput` by updating the `TextInput`'s content and selection state (when
    /// needed) and displaying a `Toast` to the user
    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let mut content = self.input.read(cx).content.clone();
        if content.len().gt(&MAX_DIGITS) {
            toast(
                cx,
                ToastVariant::Info,
                &format!("A maximum of {MAX_DIGITS} digits may be entered"),
            );

            // update input's content and relevant selection vars
            let mut truncated = content.to_string();
            truncated.truncate(MAX_DIGITS);
            content = truncated.clone().into();
            self.input.update(cx, |input, _cx| {
                input.content = truncated.into();
                input.selected_range.start = MAX_DIGITS;
                input.selected_range.end = MAX_DIGITS;
            });
        }
        let input = self.input.read(cx);
        let selected_range = input.selected_range.clone();
        let cursor = input.cursor_offset();
        let style = window.text_style();

        let (display_text, text_color) = if content.is_empty() {
            (input.placeholder.clone(), cx.theme().inactivetext)
        } else {
            (content, cx.theme().text)
        };

        let run = TextRun {
            len: display_text.len(),
            font: style.font(),
            color: text_color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let runs = if let Some(marked_range) = input.marked_range.as_ref() {
            vec![
                TextRun {
                    len: marked_range.start,
                    ..run.clone()
                },
                TextRun {
                    len: marked_range.end - marked_range.start,
                    underline: Some(UnderlineStyle {
                        color: Some(run.color),
                        thickness: px(1.0),
                        wavy: false,
                    }),
                    ..run.clone()
                },
                TextRun {
                    len: display_text.len() - marked_range.end,
                    ..run
                },
            ]
            .into_iter()
            .filter(|run| run.len > 0)
            .collect()
        } else {
            vec![run]
        };

        let font_size = style.font_size.to_pixels(window.rem_size());
        let line = window
            .text_system()
            .shape_line(display_text, font_size, &runs, None);

        let cursor_pos = line.x_for_index(cursor);
        let (selection, cursor) = if selected_range.is_empty() {
            (
                None,
                // conditionally show cursor based on blink state
                if self.input.read(cx).should_show_cursor(window, cx) {
                    Some(fill(
                        Bounds::new(
                            point(bounds.left() + cursor_pos, bounds.top()),
                            size(px(2.), bounds.bottom() - bounds.top()),
                        ),
                        cx.theme().cursor,
                    ))
                } else {
                    None
                },
            )
        } else {
            (
                Some(fill(
                    Bounds::from_corners(
                        point(
                            bounds.left() + line.x_for_index(selected_range.start),
                            bounds.top(),
                        ),
                        point(
                            bounds.left() + line.x_for_index(selected_range.end),
                            bounds.bottom(),
                        ),
                    ),
                    cx.theme().highlight,
                )),
                None,
            )
        };
        PrepaintState {
            line: Some(line),
            cursor,
            selection,
        }
    }

    /// Paint the `line`, `cursor`, and `selection` from `PrepaintState` to the screen
    /// as quads (where applicable), and handle text input via constructing an
    /// `ElementInputHandler` via the associated `TextInput`'s `focus_handle`
    ///
    /// This implementation also stores the `TextElement`'s `bounds` and `line`/`layout`
    /// in `TextInput`, which it uses to actually implement `EntityInputHandler`
    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.input.read(cx).focus_handle.clone();
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.input.clone()),
            cx,
        );
        if let Some(selection) = prepaint.selection.take() {
            window.paint_quad(selection)
        }
        let line = prepaint.line.take().unwrap_or_default();
        line.paint(bounds.origin, window.line_height(), window, cx)
            .unwrap_or_default();

        if focus_handle.is_focused(window)
            && let Some(cursor) = prepaint.cursor.take()
        {
            window.paint_quad(cursor);
        }

        self.input.update(cx, |input, _cx| {
            input.last_layout = Some(line);
            input.last_bounds = Some(bounds);
        });
    }
}
