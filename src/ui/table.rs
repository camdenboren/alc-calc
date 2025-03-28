// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

// Adapted from GPUI Example: data_table.rs

use crate::ui::input::TextInput;
use gpui::{
    canvas, div, opaque_grey, point, prelude::*, px, rgb, uniform_list, App, Bounds, Context,
    Entity, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, Render, SharedString,
    UniformListScrollHandle, Window,
};
use std::{ops::Range, rc::Rc};

const SCROLLBAR_THUMB_WIDTH: Pixels = px(8.);
const SCROLLBAR_THUMB_HEIGHT: Pixels = px(100.);

pub struct Ingredient {
    percentage_input: Entity<TextInput>,
    alc_type: SharedString,
    parts_input: Entity<TextInput>,
    weight: SharedString,
}

impl Ingredient {
    pub fn new(cx: &mut App) -> Self {
        Self {
            percentage_input: cx.new(|cx| TextInput::new(cx, "Type here...".into())),
            alc_type: String::from("Whiskey").into(),
            parts_input: cx.new(|cx| TextInput::new(cx, "Type here...".into())),
            weight: String::from("42.3").into(),
        }
    }
}

#[derive(IntoElement)]
struct TableRow {
    ix: usize,
    ingred: Rc<Ingredient>,
}
impl TableRow {
    fn new(ix: usize, ingred: Rc<Ingredient>) -> Self {
        Self { ix, ingred }
    }

    fn render_cell(&self, key: &str, width: Pixels) -> impl IntoElement {
        div()
            .whitespace_nowrap()
            .truncate()
            .w(width)
            .px_1()
            .child(match key {
                "id" => div().child(format!("{}", self.ix)),
                "alc_type" => div().child(self.ingred.alc_type.clone()),
                "percentage" => div().child(self.ingred.percentage_input.clone()),
                "parts" => div().child(self.ingred.parts_input.clone()),
                "weight" => div().child(self.ingred.weight.clone()),
                _ => div().child("--"),
            })
    }
}

const FIELDS: [(&str, f32); 5] = [
    ("id", 32.),
    ("alc_type", 96.),
    ("percentage", 128.),
    ("parts", 128.),
    ("weight", 128.),
];

impl RenderOnce for TableRow {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .border_b_1()
            .border_color(rgb(0xE0E0E0))
            .bg(if self.ix % 2 == 0 {
                opaque_grey(0.4, 1.0)
            } else {
                opaque_grey(0.6, 1.0)
            })
            .py_0p5()
            .px_2()
            .children(FIELDS.map(|(key, width)| self.render_cell(key, px(width))))
    }
}

pub struct DataTable {
    /// Use `Rc` to share the same ingred data across multiple items, avoid cloning.
    pub ingreds: Vec<Rc<Ingredient>>,
    pub visible_range: Range<usize>,
    pub scroll_handle: UniformListScrollHandle,
    /// The position in thumb bounds when dragging start mouse down.
    pub drag_position: Option<Point<Pixels>>,
}

impl DataTable {
    pub fn new() -> Self {
        Self {
            ingreds: Vec::new(),
            visible_range: 0..0,
            scroll_handle: UniformListScrollHandle::new(),
            drag_position: None,
        }
    }

    pub fn generate(&mut self, num_ingredients: i32, cx: &mut App) {
        self.ingreds = (0..num_ingredients)
            .map(|_| Rc::new(Ingredient::new(cx)))
            .collect();
    }

    fn table_bounds(&self) -> Bounds<Pixels> {
        self.scroll_handle.0.borrow().base_handle.bounds()
    }

    fn scroll_top(&self) -> Pixels {
        self.scroll_handle.0.borrow().base_handle.offset().y
    }

    fn scroll_height(&self) -> Pixels {
        self.scroll_handle
            .0
            .borrow()
            .last_item_size
            .unwrap_or_default()
            .contents
            .height
    }

    fn render_scrollbar(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let scroll_height = self.scroll_height();
        let table_bounds = self.table_bounds();
        let table_height = table_bounds.size.height;
        if table_height == px(0.) {
            return div().id("scrollbar");
        }

        let percentage = -self.scroll_top() / scroll_height;
        let offset_top = (table_height * percentage).clamp(
            px(4.),
            (table_height - SCROLLBAR_THUMB_HEIGHT - px(4.)).max(px(4.)),
        );
        let entity = cx.entity();
        let scroll_handle = self.scroll_handle.0.borrow().base_handle.clone();

        div()
            .id("scrollbar")
            .absolute()
            .top(offset_top)
            .right_1()
            .h(SCROLLBAR_THUMB_HEIGHT)
            .w(SCROLLBAR_THUMB_WIDTH)
            .bg(rgb(0xC0C0C0))
            .hover(|this| this.bg(rgb(0xA0A0A0)))
            .rounded_lg()
            .child(
                canvas(
                    |_, _, _| (),
                    move |thumb_bounds, _, window, _| {
                        window.on_mouse_event({
                            let entity = entity.clone();
                            move |ev: &MouseDownEvent, _, _, cx| {
                                if !thumb_bounds.contains(&ev.position) {
                                    return;
                                }

                                entity.update(cx, |this, _| {
                                    this.drag_position = Some(
                                        ev.position - thumb_bounds.origin - table_bounds.origin,
                                    );
                                })
                            }
                        });
                        window.on_mouse_event({
                            let entity = entity.clone();
                            move |_: &MouseUpEvent, _, _, cx| {
                                entity.update(cx, |this, _| {
                                    this.drag_position = None;
                                })
                            }
                        });

                        window.on_mouse_event(move |ev: &MouseMoveEvent, _, _, cx| {
                            if !ev.dragging() {
                                return;
                            }

                            let Some(drag_pos) = entity.read(cx).drag_position else {
                                return;
                            };

                            let inside_offset = drag_pos.y;
                            let percentage = ((ev.position.y - table_bounds.origin.y
                                + inside_offset)
                                / (table_bounds.size.height))
                                .clamp(0., 1.);

                            let offset_y = ((scroll_height - table_bounds.size.height)
                                * percentage)
                                .clamp(px(0.), scroll_height - SCROLLBAR_THUMB_HEIGHT);
                            scroll_handle.set_offset(point(px(0.), -offset_y));
                            cx.notify(entity.entity_id());
                        })
                    },
                )
                .size_full(),
            )
    }
}

impl Render for DataTable {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();

        div()
            .bg(opaque_grey(0.2, 1.0))
            .text_sm()
            .size_full()
            .p_4()
            .gap_2()
            .flex()
            .flex_col()
            .max_w_1_2()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_hidden()
                    .border_1()
                    .border_color(rgb(0xE0E0E0))
                    .rounded_sm()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .w_full()
                            .overflow_hidden()
                            .border_b_1()
                            .border_color(rgb(0xE0E0E0))
                            .text_color(rgb(0xffffff))
                            .bg(opaque_grey(0.2, 1.0))
                            .py_1()
                            .px_2()
                            .text_xs()
                            .children(FIELDS.map(|(key, width)| {
                                div()
                                    .whitespace_nowrap()
                                    .flex_shrink_0()
                                    .truncate()
                                    .px_1()
                                    .w(px(width))
                                    .child(key.replace("_", " ").to_uppercase())
                            })),
                    )
                    .child(
                        div()
                            .relative()
                            .size_full()
                            .child(
                                uniform_list(entity, "items", self.ingreds.len(), {
                                    move |this, range, _, _| {
                                        this.visible_range = range.clone();
                                        let mut items = Vec::with_capacity(range.end - range.start);
                                        for i in range {
                                            if let Some(ingred) = this.ingreds.get(i) {
                                                items.push(TableRow::new(i, ingred.clone()));
                                            }
                                        }
                                        items
                                    }
                                })
                                .size_full()
                                .track_scroll(self.scroll_handle.clone()),
                            )
                            .child(self.render_scrollbar(window, cx)),
                    ),
            )
    }
}
