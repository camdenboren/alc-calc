// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ui::dropdown::Dropdown;
use crate::ui::input::TextInput;
use gpui::{div, opaque_grey, prelude::*, px, App, Entity, Pixels, SharedString, Window};

pub const FIELDS: [(&str, f32); 4] = [
    ("alc_type", 132.),
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
