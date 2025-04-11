// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

// Adapted from GPUI Example: data_table.rs

use crate::ui::card::card;
use crate::ui::ingredient::{Ingredient, IngredientData, FIELDS};
use gpui::{div, opaque_grey, prelude::*, px, rgb, App, Entity, Window};

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

    fn ready(&mut self, cx: &mut App) -> (bool, Vec<IngredientData>) {
        let mut ready = true;
        let mut ingred_data: Vec<IngredientData> = Vec::new();

        for ingred in &self.ingreds {
            let percentage = ingred.read(cx).percentage_input.read(cx).content.clone();
            let parts = ingred.read(cx).parts_input.read(cx).content.clone();
            let percentage: f32 = match percentage.trim().parse() {
                Ok(num) => num,
                Err(_) => 0.,
            };
            let parts: f32 = match parts.trim().parse() {
                Ok(num) => num,
                Err(_) => 0.,
            };
            if percentage <= 0. || parts <= 0. {
                ready = false;
                return (ready, vec![]);
            }
            let mut data = IngredientData::new();
            data.percentage = percentage;
            data.parts = parts;
            ingred_data.push(data);
        }
        (ready, ingred_data)
    }

    fn calc(&mut self, cx: &mut App, data: &mut Vec<IngredientData>) {
        let mut ix = 0;
        for ingred in &self.ingreds {
            data[ix].alc_type = ingred.read(cx).alc_type.read(cx).current.clone();
        }

        ix = 0;
        for ingred in &self.ingreds {
            let weight = data[ix].weight.clone();
            ingred.update(cx, |ingred, _| {
                ingred.weight(weight);
            });
            ix += 1;
        }
    }
}

impl Render for Table {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let (ready, mut data) = self.ready(cx);
        if ready {
            self.calc(cx, &mut data);
        }

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
