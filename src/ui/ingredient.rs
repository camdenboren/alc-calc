// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ui::button::button;
use crate::ui::dropdown::Dropdown;
use crate::ui::input::TextInput;
use gpui::{div, opaque_grey, prelude::*, px, App, Entity, Pixels, SharedString, Window};

pub const FIELDS: [(&str, f32); 4] = [
    ("alc_type", 148.),
    ("percentage", 132.),
    ("parts", 132.),
    ("weight", 72.),
];

pub struct Ingredient {
    pub alc_type: Entity<Dropdown>,
    pub percentage_input: Entity<TextInput>,
    pub parts_input: Entity<TextInput>,
    pub weight: SharedString,
    pub remove: bool,
}

impl Ingredient {
    pub fn new(id: usize, cx: &mut App) -> Self {
        Self {
            alc_type: cx.new(|cx| Dropdown::new(id, cx)),
            percentage_input: cx.new(|cx| TextInput::new(cx, "Type here...".into())),
            parts_input: cx.new(|cx| TextInput::new(cx, "Type here...".into())),
            weight: String::from("0").into(),
            remove: false,
        }
    }

    fn render_cell(&self, key: &str, width: Pixels) -> impl IntoElement {
        div().w(width).child(match key {
            "alc_type" => div().child(self.alc_type.clone()),
            "percentage" => div().child(self.percentage_input.clone()),
            "parts" => div().child(self.parts_input.clone()),
            "weight" => div()
                .flex()
                .flex_row()
                .child(self.weight.clone())
                .when(&self.weight != "--", |this| this.child("g")),
            _ => div().child("--"),
        })
    }

    pub fn weight(&mut self, weight: f32) {
        self.weight = SharedString::from(weight.to_string());
    }

    fn remove(&mut self) {
        self.remove = true;
    }
}

impl Render for Ingredient {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .border_b_1()
            .border_color(opaque_grey(0.5, 0.5))
            .py_1()
            .items_center()
            .justify_center()
            .gap_x_4()
            .child(button(
                "",
                "minus.svg",
                cx.listener(move |this, _, _window, _cx| this.remove()),
            ))
            .children(FIELDS.map(|(key, width)| self.render_cell(key, px(width))))
    }
}

#[derive(Clone)]
pub struct IngredientData {
    pub alc_type: SharedString,
    pub percentage: f32,
    pub parts: f32,
    pub density: f32,
    pub volume: f32,
    pub weight: f32,
    pub intermediate_weight: f32,
}

impl IngredientData {
    fn new() -> Self {
        Self {
            alc_type: "".into(),
            percentage: 0.,
            parts: 0.,
            density: 0.,
            volume: 0.,
            weight: 0.,
            intermediate_weight: 0.,
        }
    }
}

impl Default for IngredientData {
    fn default() -> Self {
        Self::new()
    }
}
