// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

// Adapted from: https://github.com/zed-industries/zed/blob/main/crates/gpui/examples/data_table.rs

use crate::ui::{
    ActiveCtrl,
    comp::{
        button::icon_button,
        dropdown::Dropdown,
        icon::{Icon, IconSize, IconVariant},
        input::text_input::TextInput,
        tooltip::Tooltip,
    },
    util::theme::ActiveTheme,
};
use gpui::{Entity, EventEmitter, Pixels, SharedString, Window, div, prelude::*, px};

pub const FIELDS: [(&str, &str, f32); 4] = [
    ("ingredient", "Type of ingredient (e.g., Whiskey)", 148.),
    (
        "percentage",
        "Percentage of alcohol in the ingredient",
        132.,
    ),
    (
        "parts",
        "Desired number of parts of this ingredient relative to others",
        132.,
    ),
    (
        "weight",
        "Calculated weight (in g) of this ingredient to pour in the drink",
        72.,
    ),
];

pub struct Ingredient {
    pub ingred_type: Entity<Dropdown>,
    pub percentage_input: Entity<TextInput>,
    pub parts_input: Entity<TextInput>,
    pub weight: SharedString,
    pub id: usize,
}

impl Ingredient {
    pub fn new(id: usize, window: &mut Window, cx: &mut Context<Self>) -> Self {
        // we have 3 items per indgred and tab_index 1 is num_drinks_input,
        // so multiply by 3 and offset by two (ui itself is tab_index 0)
        Self {
            ingred_type: cx.new(|cx| Dropdown::new(id, cx, id as isize * 3 + 2)),
            percentage_input: cx
                .new(|cx| TextInput::new(window, cx, "Type here...".into(), id as isize * 3 + 3)),
            parts_input: cx
                .new(|cx| TextInput::new(window, cx, "Type here...".into(), id as isize * 3 + 4)),
            weight: "0".into(),
            id,
        }
    }

    fn render_cell(&self, key: &str, width: Pixels) -> impl IntoElement {
        div().w(width).child(match key {
            "ingredient" => div().id("").child(self.ingred_type.clone()),
            "percentage" => div().id("").child(self.percentage_input.clone()),
            "parts" => div().id("").child(self.parts_input.clone()),
            "weight" => {
                let display_weight = self.weight.to_string() + "g";
                div()
                    .w(width) // needs to be set again to inform truncate() of width
                    .truncate()
                    .child(display_weight.clone())
                    .id(format!("{}-weight", self.id).into_element())
                    .tooltip(move |_window, cx| cx.new(|_cx| Tooltip::new(&display_weight)).into())
            }
            _ => div().id("").child("--"),
        })
    }

    pub fn weight(&mut self, weight: f32) {
        self.weight = weight.to_string().into();
    }

    pub fn show_cursor_and_hide_dd(&mut self, cx: &mut Context<Self>) {
        self.ingred_type
            .update(cx, |ingred_type, cx| ingred_type.hide(cx));
        self.percentage_input
            .update(cx, |percentage, cx| percentage.show_cursor(cx));
        self.parts_input
            .update(cx, |parts, cx| parts.show_cursor(cx));
    }

    fn remove(&mut self, cx: &mut Context<Self>) {
        cx.emit(Remove {});
    }
}

impl Render for Ingredient {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let id = self.id;

        div()
            .flex()
            .flex_row()
            .border_b_1()
            .border_color(cx.theme().background)
            .py_1()
            .items_center()
            .justify_center()
            .gap_x_4()
            .child(
                div()
                    .flex()
                    .child(icon_button(
                        "remove",
                        Icon::new(cx, IconVariant::Minus, IconSize::Small),
                        cx.listener(move |this, _, _window, cx| this.remove(cx)),
                    ))
                    .id(format!("remove_button_{id}").into_element())
                    .tooltip(|_window, cx| {
                        cx.new(|cx| {
                            Tooltip::new("Remove this Ingredient")
                                .keybind(&format!("{}-r", cx.ctrl()))
                        })
                        .into()
                    }),
            )
            .children(FIELDS.map(|(key, _, width)| self.render_cell(key, px(width))))
    }
}

pub struct Remove {}

impl EventEmitter<Remove> for Ingredient {}

#[derive(Clone)]
pub struct IngredientData {
    pub ingred_type: SharedString,
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
            ingred_type: "".into(),
            percentage: 0.,
            parts: 0.,
            density: 0.,
            volume: 0.,
            weight: 0.,
            intermediate_weight: 0.,
        }
    }
}
