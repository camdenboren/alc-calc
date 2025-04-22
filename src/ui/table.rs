// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::calc::calc_weights;
use crate::ui::button::button;
use crate::ui::ingredient::{Ingredient, IngredientData, FIELDS};
use crate::ui::input::TextInput;
use gpui::{
    actions, div, opaque_grey, prelude::*, px, rgb, App, Entity, FocusHandle, Focusable,
    KeyBinding, Window,
};
use std::env::consts::OS;

actions!(table, [Tab, Add, Delete, Escape]);

pub const MAX_ITEMS: usize = 10;

pub struct Table {
    pub ingreds: Vec<Entity<Ingredient>>,
    pub num_drinks_input: Entity<TextInput>,
    pub num_drinks: f32,
    width: f32,
    init: bool,
    focus_handle: FocusHandle,
}

impl Table {
    pub fn new(cx: &mut App) -> Self {
        let mut width = 0.;
        for field in FIELDS {
            let (_, val) = field;
            width += val;
        }

        Self {
            ingreds: vec![cx.new(|cx| Ingredient::new(0, cx))],
            num_drinks_input: cx.new(|cx| TextInput::new(cx, "Type here...".into())),
            num_drinks: 0.,
            width,
            init: true,
            focus_handle: cx.focus_handle(),
        }
    }

    fn add(&mut self, _: &Add, _window: &mut Window, cx: &mut Context<Self>) {
        if self.ingreds.len() < MAX_ITEMS {
            let id = self.ingreds.len();
            self.ingreds.push(cx.new(|cx| Ingredient::new(id, cx)))
        }
    }

    fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        // move focus to num_drinks_input when focused ingredient is deleted
        let len = self.ingreds.len();
        if len > 0 {
            if self.ingreds[len - 1]
                .read(cx)
                .parts_input
                .focus_handle(cx)
                .is_focused(window)
                || self.ingreds[len - 1]
                    .read(cx)
                    .percentage_input
                    .focus_handle(cx)
                    .is_focused(window)
                || self.ingreds[len - 1]
                    .read(cx)
                    .alc_type
                    .focus_handle(cx)
                    .is_focused(window)
            {
                self.focus_handle(cx).focus(window);
            }
            self.ingreds.pop();
        }
    }

    pub fn remove(&mut self, ix: usize) {
        self.ingreds.remove(ix);
    }

    fn ready(&mut self, cx: &mut App) -> (bool, Vec<IngredientData>) {
        let mut ready = true;
        let mut ingred_data: Vec<IngredientData> = Vec::new();

        // calc_weights requires non-zero vec
        if self.ingreds.len() < 1 {
            ready = false;
            return (ready, vec![]);
        }

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
            if percentage <= 0. || (self.ingreds.len() > 1 && parts <= 0.) {
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

    fn calc(&mut self, cx: &mut App, data: &mut Vec<IngredientData>, num_drinks: f32) {
        let mut ix = 0;
        for ingred in &self.ingreds {
            data[ix].alc_type = ingred.read(cx).alc_type.read(cx).current.clone();
            ix += 1;
        }

        let data = calc_weights(data, num_drinks);

        ix = 0;
        for ingred in &self.ingreds {
            let weight = data[ix].weight.clone();
            ingred.update(cx, |ingred, _| {
                ingred.weight(weight);
            });
            ix += 1;
        }
    }

    fn focus_self(&mut self, _: &Escape, window: &mut Window, _cx: &mut Context<Self>) {
        self.focus_handle.focus(window);
    }

    fn focus_next(&mut self, _: &Tab, window: &mut Window, cx: &mut Context<Self>) {
        // return early for base cases (e.g. entering or leaving ingreds list)
        let len = self.ingreds.len();
        if self.focus_handle.is_focused(window) {
            self.num_drinks_input.focus_handle(cx).focus(window);
            return;
        }
        if self.num_drinks_input.focus_handle(cx).is_focused(window) && len > 0 {
            self.ingreds[0]
                .read(cx)
                .alc_type
                .focus_handle(cx)
                .focus(window);
            return;
        }
        if len > 0 {
            if self.ingreds[len - 1]
                .read(cx)
                .parts_input
                .focus_handle(cx)
                .is_focused(window)
            {
                self.num_drinks_input.focus_handle(cx).focus(window);
                return;
            }
        }

        // focus next ingred field otw
        for ix in 0..len {
            if self.ingreds[ix]
                .read(cx)
                .alc_type
                .focus_handle(cx)
                .is_focused(window)
            {
                // hide dropdown before focusing input
                if self.ingreds[ix].read(cx).alc_type.read(cx).show {
                    self.ingreds[ix]
                        .read(cx)
                        .alc_type
                        .clone()
                        .update(cx, |alc_type, _| alc_type.toggle())
                }
                self.ingreds[ix]
                    .read(cx)
                    .percentage_input
                    .focus_handle(cx)
                    .focus(window);
                break;
            } else if self.ingreds[ix]
                .read(cx)
                .percentage_input
                .focus_handle(cx)
                .is_focused(window)
            {
                self.ingreds[ix]
                    .read(cx)
                    .parts_input
                    .focus_handle(cx)
                    .focus(window);
                break;
            } else if len > ix + 1
                && self.ingreds[ix]
                    .read(cx)
                    .parts_input
                    .focus_handle(cx)
                    .is_focused(window)
            {
                self.ingreds[ix + 1]
                    .read(cx)
                    .alc_type
                    .focus_handle(cx)
                    .focus(window);
                break;
            }
        }
    }
}

impl Render for Table {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // focus num_drinks_input on launch
        if self.init {
            self.num_drinks_input.focus_handle(cx).focus(window);
            self.init = false;
        }

        let ctrl = if OS == "linux" { "ctrl" } else { "cmd" };
        cx.bind_keys([
            KeyBinding::new("tab", Tab, None),
            KeyBinding::new(format!("{ctrl}-i").as_str(), Add, None),
            KeyBinding::new(format!("{ctrl}-d").as_str(), Delete, None),
            KeyBinding::new(format!("escape").as_str(), Escape, None),
        ]);

        let num_drinks = self.num_drinks_input.read(cx).content.clone();
        let num_drinks: f32 = match num_drinks.trim().parse() {
            Ok(num) => num,
            Err(_) => 0.,
        };
        self.num_drinks = num_drinks;

        let mut ix = 0;
        for ingred in &self.ingreds.clone() {
            if ingred.read(cx).remove {
                self.remove(ix)
            }
            ix += 1;
        }

        let (ready, mut data) = self.ready(cx);
        if ready {
            self.calc(cx, &mut data, self.num_drinks);
        }

        div()
            .key_context("Table")
            .on_action(cx.listener(Self::focus_next))
            .on_action(cx.listener(Self::focus_self))
            .on_action(cx.listener(Self::add))
            .on_action(cx.listener(Self::delete))
            .track_focus(&self.focus_handle(cx))
            .flex()
            .flex_col()
            .gap_3()
            .items_center()
            // num_drinks input
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_shrink()
                    .p_4()
                    .justify_center()
                    .items_center()
                    .rounded_lg()
                    .bg(opaque_grey(0.2, 1.0))
                    .gap_1()
                    .child(
                        div()
                            .flex()
                            .py_2()
                            .text_xs()
                            .border_b_1()
                            .justify_start()
                            .w(px(120. + 4. * 2.))
                            .border_color(opaque_grey(0.5, 0.5))
                            .child("Units".to_uppercase()),
                    )
                    .child(self.num_drinks_input.clone()),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .p_4()
                    .justify_center()
                    .items_center()
                    .gap_2()
                    .bg(opaque_grey(0.2, 1.0))
                    .rounded_lg()
                    // header
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_center()
                            .gap_x_4()
                            .overflow_hidden()
                            .text_color(rgb(0xffffff))
                            .bg(opaque_grey(0.2, 1.0))
                            .left_4()
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
                    )
                    // + button
                    .child(div().py_1().w(px(self.width + 78.)).child(button(
                        "",
                        "plus.svg",
                        cx.listener(move |this, _, window, cx| {
                            this.add(&Add, window, cx);
                        }),
                    ))),
            )
    }
}

impl Focusable for Table {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
