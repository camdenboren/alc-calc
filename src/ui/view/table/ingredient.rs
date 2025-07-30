// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

// Adapted from: https://github.com/zed-industries/zed/blob/main/crates/gpui/examples/data_table.rs

use crate::ui::{
    comp::{
        button::button,
        dropdown::Dropdown,
        icon::{Icon, IconSize, IconVariant},
        input::text_input::TextInput,
    },
    util::theme::ActiveTheme,
};
use gpui::{Entity, EventEmitter, Pixels, SharedString, Window, div, prelude::*, px};

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
    pub id: usize,
}

impl Ingredient {
    pub fn new(id: usize, window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            alc_type: cx.new(|cx| Dropdown::new(id, cx)),
            percentage_input: cx.new(|cx| TextInput::new(window, cx, "Type here...".into())),
            parts_input: cx.new(|cx| TextInput::new(window, cx, "Type here...".into())),
            weight: "0".into(),
            id,
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
        self.weight = weight.to_string().into();
    }

    fn remove(&mut self, cx: &mut Context<Self>) {
        cx.emit(Remove {});
    }
}

impl Render for Ingredient {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .border_b_1()
            .border_color(cx.theme().background)
            .py_1()
            .items_center()
            .justify_center()
            .gap_x_4()
            .child(button(
                "remove",
                Icon::new(IconVariant::Minus, IconSize::Small),
                cx,
                cx.listener(move |this, _, _window, cx| this.remove(cx)),
            ))
            .children(FIELDS.map(|(key, width)| self.render_cell(key, px(width))))
    }
}

pub struct Remove {}

impl EventEmitter<Remove> for Ingredient {}

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

impl Default for IngredientData {
    fn default() -> Self {
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
