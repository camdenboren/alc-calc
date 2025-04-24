// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    calc::calc_weights,
    ui::{
        button::button,
        dropdown::Dropdown,
        ingredient::{Ingredient, IngredientData, FIELDS},
        input::TextInput,
    },
};
use gpui::{
    actions, div, opaque_grey, prelude::*, px, rgb, App, Entity, FocusHandle, Focusable,
    KeyBinding, SharedString, Window,
};
use std::env::consts::OS;

actions!(table, [Tab, Add, Delete, Escape]);

pub const MAX_ITEMS: usize = 10;

pub struct Table {
    ingreds: Vec<Entity<Ingredient>>,
    num_drinks_input: Entity<TextInput>,
    num_drinks: f32,
    count: usize,
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
            count: 1,
            width,
            init: true,
            focus_handle: cx.focus_handle(),
        }
    }

    fn add(&mut self, _: &Add, _window: &mut Window, cx: &mut Context<Self>) {
        if self.count < MAX_ITEMS {
            let id = self.count;
            self.ingreds.push(cx.new(|cx| Ingredient::new(id, cx)));
            self.count += 1;
        }
    }

    fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        // move focus to num_drinks_input when focused ingredient is deleted
        if self.count > 0 {
            if self.parts(self.count - 1, cx).is_focused(window)
                || self.percentage(self.count - 1, cx).is_focused(window)
                || self.alc_type(self.count - 1, cx).is_focused(window)
            {
                self.focus_handle(cx).focus(window);
            }
            self.ingreds.pop();
            self.count -= 1;
        }
    }

    fn remove(&mut self, ix: usize) {
        self.ingreds.remove(ix);
        self.count -= 1;
    }

    fn ready(&mut self, cx: &mut Context<Self>) -> bool {
        // calc_weights requires non-zero vec
        if self.ingreds.is_empty() {
            return false;
        }

        (0..self.count).all(|ix| {
            let percentage = self.parse_or_zero(self.percentage(ix, cx).content.clone());
            let parts = self.parse_or_zero(self.parts(ix, cx).content.clone());
            percentage > 0. && (self.count <= 1 || parts > 0.)
        })
    }

    fn calc(&mut self, cx: &mut Context<Self>, num_drinks: f32) {
        let mut ingred_data: Vec<IngredientData> = (0..self.count)
            .map(|ix| IngredientData {
                alc_type: self.alc_type(ix, cx).current.clone(),
                percentage: self.parse_or_zero(self.percentage(ix, cx).content.clone()),
                parts: self.parse_or_zero(self.parts(ix, cx).content.clone()),
                ..Default::default()
            })
            .collect();

        let ingred_data = calc_weights(&mut ingred_data, num_drinks);

        self.ingreds.iter().enumerate().for_each(|(ix, ingred)| {
            ingred.update(cx, |ingred, _| {
                ingred.weight(ingred_data[ix].weight);
            });
        })
    }

    fn num_drinks<'a>(&'a self, cx: &'a Context<Self>) -> &'a TextInput {
        self.num_drinks_input.read(cx)
    }

    fn alc_type<'a>(&'a self, ix: usize, cx: &'a Context<Self>) -> &'a Dropdown {
        self.ingreds[ix].read(cx).alc_type.read(cx)
    }

    fn parts<'a>(&'a self, ix: usize, cx: &'a Context<Self>) -> &'a TextInput {
        self.ingreds[ix].read(cx).parts_input.read(cx)
    }

    fn percentage<'a>(&'a self, ix: usize, cx: &'a Context<Self>) -> &'a TextInput {
        self.ingreds[ix].read(cx).percentage_input.read(cx)
    }

    fn parse_or_zero(&mut self, content: SharedString) -> f32 {
        content.trim().parse().unwrap_or(0.)
    }

    fn focus(&mut self, _: &Escape, window: &mut Window, _cx: &mut Context<Self>) {
        self.focus_handle.focus(window);
    }

    fn is_focused(&self, window: &mut Window) -> bool {
        self.focus_handle.is_focused(window)
    }

    fn focus_next(&mut self, _: &Tab, window: &mut Window, cx: &mut Context<Self>) {
        // return early for base cases (e.g. entering or leaving ingreds list)
        if self.is_focused(window) {
            self.num_drinks(cx).focus(window);
            return;
        }
        if self.num_drinks(cx).is_focused(window) && self.count > 0 {
            self.alc_type(0, cx).focus(window);
            return;
        }
        if self.count > 0 && self.parts(self.count - 1, cx).is_focused(window) {
            self.num_drinks(cx).focus(window);
            return;
        }

        // focus next ingred field otw
        for ix in 0..self.count {
            if self.alc_type(ix, cx).is_focused(window) {
                // hide dropdown before focusing input
                if self.alc_type(ix, cx).show {
                    self.ingreds[ix]
                        .read(cx)
                        .alc_type
                        .clone()
                        .update(cx, |alc_type, _| alc_type.toggle())
                }
                self.percentage(ix, cx).focus(window);
                break;
            } else if self.percentage(ix, cx).is_focused(window) {
                self.parts(ix, cx).focus(window);
                break;
            } else if self.count > ix + 1 && self.parts(ix, cx).is_focused(window) {
                self.alc_type(ix + 1, cx).focus(window);
                break;
            }
        }
    }
}

impl Render for Table {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // focus num_drinks_input on launch
        if self.init {
            self.num_drinks(cx).focus(window);
            self.init = false;
        }

        let ctrl = if OS == "linux" { "ctrl" } else { "cmd" };
        cx.bind_keys([
            KeyBinding::new("tab", Tab, None),
            KeyBinding::new(format!("{ctrl}-i").as_str(), Add, None),
            KeyBinding::new(format!("{ctrl}-d").as_str(), Delete, None),
            KeyBinding::new("escape", Escape, None),
        ]);

        self.num_drinks = self.parse_or_zero(self.num_drinks(cx).content.clone());

        self.ingreds
            .clone()
            .iter()
            .enumerate()
            .for_each(|(ix, item)| {
                if item.read(cx).remove {
                    self.remove(ix)
                }
            });

        if self.ready(cx) {
            self.calc(cx, self.num_drinks);
        }

        div()
            .key_context("Table")
            .on_action(cx.listener(Self::focus_next))
            .on_action(cx.listener(Self::focus))
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
