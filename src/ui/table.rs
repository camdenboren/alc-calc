// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

// Adapted from GPUI Example: data_table.rs

use crate::ui::card::card;
use crate::ui::dropdown::Dropdown;
use crate::ui::input::TextInput;
use gpui::{div, opaque_grey, prelude::*, px, rgb, App, Entity, Pixels, SharedString, Window};

pub const FIELDS: [(&str, f32); 4] = [
    ("alc_type", 138.),
    ("percentage", 132.),
    ("parts", 132.),
    ("weight", 72.),
];

pub struct Ingredient {
    alc_type: Entity<Dropdown>,
    percentage_input: Entity<TextInput>,
    parts_input: Entity<TextInput>,
    weight: SharedString,
}

impl Ingredient {
    pub fn new(cx: &mut App) -> Self {
        Self {
            alc_type: cx.new(|_| Dropdown::new()),
            percentage_input: cx.new(|cx| TextInput::new(cx, "Type here...".into())),
            parts_input: cx.new(|cx| TextInput::new(cx, "Type here...".into())),
            weight: String::from("42.3").into(),
        }
    }

    fn render_cell(&self, key: &str, width: Pixels) -> impl IntoElement {
        div().w(width).child(match key {
            "alc_type" => div().child(self.alc_type.clone()),
            "percentage" => div().child(self.percentage_input.clone()),
            "parts" => div().child(self.parts_input.clone()),
            "weight" => div().child(self.weight.clone()),
            _ => div().child("--"),
        })
    }
}

impl Render for Ingredient {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .border_b_1()
            .border_color(opaque_grey(0.5, 0.5))
            .py_1()
            .items_center()
            .justify_center()
            .gap_x_4()
            .children(FIELDS.map(|(key, width)| self.render_cell(key, px(width))))
    }
}

pub struct Table {
    pub ingreds: Vec<Entity<Ingredient>>,
}

impl Table {
    pub fn new() -> Self {
        Self { ingreds: vec![] }
    }

    pub fn refresh(&mut self, cx: &mut App, num_ingredients: i32) {
        self.ingreds = vec![];
        for _ in 0..num_ingredients {
            self.ingreds.push(cx.new(|cx| Ingredient::new(cx)))
        }
    }
}

impl Render for Table {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        card(
            div()
                .flex()
                .flex_col()
                .rounded_sm()
                // header
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_center()
                        .w_full()
                        .gap_x_4()
                        .overflow_hidden()
                        .text_color(rgb(0xffffff))
                        .bg(opaque_grey(0.2, 1.0))
                        .py_1()
                        .text_xs()
                        .children(FIELDS.map(|(key, width)| {
                            div()
                                .whitespace_nowrap()
                                .flex_shrink_0()
                                .truncate()
                                .w(px(width))
                                .child(key.replace("_", " ").to_uppercase())
                        })),
                )
                // ingreds
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .border_t_1()
                        .border_color(opaque_grey(0.5, 0.5))
                        .children(self.ingreds.clone()),
                ),
        )
    }
}
