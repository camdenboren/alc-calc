// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

// Adapted from: https://github.com/zed-industries/zed/blob/main/crates/gpui/examples/data_table.rs

use crate::{
    calc::calc_weights,
    ui::{
        button::button,
        dropdown::Dropdown,
        icon::{Icon, IconSize, IconVariant},
        input::TextInput,
        theme::ActiveTheme,
    },
};
use gpui::{
    App, Entity, EventEmitter, FocusHandle, Focusable, KeyBinding, Pixels, SharedString, Window,
    actions, div, prelude::*, px,
};

actions!(table, [Tab, Add, Delete, Escape, RemoveKey]);

const CONTEXT: &str = "Table";
pub const MAX_ITEMS: usize = 10;
const FIELDS: [(&str, f32); 4] = [
    ("alc_type", 148.),
    ("percentage", 132.),
    ("parts", 132.),
    ("weight", 72.),
];

struct Ingredient {
    pub alc_type: Entity<Dropdown>,
    pub percentage_input: Entity<TextInput>,
    pub parts_input: Entity<TextInput>,
    weight: SharedString,
    pub id: usize,
}

impl Ingredient {
    pub fn new(id: usize, cx: &mut App) -> Self {
        let is_linux = cfg!(target_os = "linux");
        let ctrl = if is_linux { "ctrl" } else { "cmd" };
        cx.bind_keys([
            KeyBinding::new("tab", Tab, Some(CONTEXT)),
            KeyBinding::new(format!("{ctrl}-i").as_str(), Add, Some(CONTEXT)),
            KeyBinding::new(format!("{ctrl}-d").as_str(), Delete, Some(CONTEXT)),
            KeyBinding::new(format!("{ctrl}-r").as_str(), RemoveKey, Some(CONTEXT)),
            KeyBinding::new("escape", Escape, Some(CONTEXT)),
        ]);

        Self {
            alc_type: cx.new(|cx| Dropdown::new(id, cx)),
            percentage_input: cx.new(|cx| TextInput::new(cx, "Type here...".into())),
            parts_input: cx.new(|cx| TextInput::new(cx, "Type here...".into())),
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

struct Remove {}

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

pub struct Table {
    ingreds: Vec<Entity<Ingredient>>,
    pub num_drinks_input: Entity<TextInput>,
    num_drinks: f32,
    count: usize,
    width: f32,
    init: bool,
    focus_handle: FocusHandle,
}

impl Table {
    pub fn new(cx: &mut App) -> Self {
        Self {
            ingreds: vec![],
            num_drinks_input: cx.new(|cx| TextInput::new(cx, "Type here...".into())),
            num_drinks: 0.,
            count: 0,
            width: FIELDS.iter().fold(0., |acc, field| acc + field.1),
            init: true,
            focus_handle: cx.focus_handle(),
        }
    }

    fn add(&mut self, _: &Add, _window: &mut Window, cx: &mut Context<Self>) {
        if self.count < MAX_ITEMS {
            let id = self.count;
            let ingred = cx.new(|cx| Ingredient::new(id, cx));
            self.ingreds.push(ingred.clone());
            self.count += 1;

            // subscribe to Ingred's Remove event
            cx.subscribe(
                &ingred,
                |this: &mut Table, ingred: Entity<Ingredient>, _event, cx| {
                    this.remove(ingred.read(cx).id, cx)
                },
            )
            .detach();
        }
        cx.notify();
    }

    fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        if self.count > 0 {
            if self.parts(self.count - 1, cx).is_focused(window)
                || self.percentage(self.count - 1, cx).is_focused(window)
                || self.alc_type(self.count - 1, cx).is_focused(window)
            {
                self.focus(&Escape, window, cx);
            }
            self.ingreds.pop();
            self.count -= 1;
        }
        cx.notify();
    }

    fn remove(&mut self, ix: usize, cx: &mut Context<Self>) {
        self.ingreds.remove(ix);
        self.count -= 1;

        // update id's so that we can use them for indexed removal and dd deferral
        self.ingreds[ix..]
            .iter()
            .enumerate()
            .for_each(|(jx, ingred)| {
                ingred.update(cx, |ingred, cx| {
                    ingred.id = jx + ix;
                    ingred.alc_type.update(cx, |alc_type, _cx| {
                        alc_type.id = jx + ix;
                    });
                })
            });
    }

    fn remove_key(&mut self, _: &RemoveKey, window: &mut Window, cx: &mut Context<Self>) {
        if self.count > 0 {
            for ix in 0..self.count {
                if self.alc_type(ix, cx).is_focused(window)
                    || self.parts(ix, cx).is_focused(window)
                    || self.percentage(ix, cx).is_focused(window)
                {
                    self.remove(ix, cx);
                    self.focus(&Escape, window, cx);
                    break;
                }
            }
        }
        cx.notify();
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

        let ingred_data = match calc_weights(&mut ingred_data, num_drinks) {
            Ok(ingred_data) => ingred_data,
            Err(_) => return,
        };

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
                        .update(cx, |alc_type, cx| alc_type.toggle(cx))
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
        // focus num_drinks_input and add ingred on launch
        if self.init {
            self.add(&Add, window, cx);
            self.num_drinks(cx).focus(window);
            self.init = false;
        }

        self.num_drinks = self.parse_or_zero(self.num_drinks(cx).content.clone());

        if self.ready(cx) {
            self.calc(cx, self.num_drinks);
        }

        div()
            .key_context(CONTEXT)
            .on_action(cx.listener(Self::focus_next))
            .on_action(cx.listener(Self::focus))
            .on_action(cx.listener(Self::add))
            .on_action(cx.listener(Self::delete))
            .on_action(cx.listener(Self::remove_key))
            .track_focus(&self.focus_handle(cx))
            .flex()
            .flex_col()
            .bottom(px(55.))
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
                    .bg(cx.theme().foreground)
                    .gap_1()
                    .child(
                        div()
                            .flex()
                            .bottom(px(0.5))
                            .pb_2()
                            .text_xs()
                            .border_b_1()
                            .justify_start()
                            .w(px(120. + 4. * 2.))
                            .border_color(cx.theme().background)
                            .child(div().child("Units".to_uppercase()).bottom(px(1.5))),
                    )
                    .child(self.num_drinks_input.clone()),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .p_4()
                    .items_center()
                    .gap_2()
                    .bg(cx.theme().foreground)
                    .rounded_lg()
                    // header
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .h_5()
                            .gap_x_4()
                            .overflow_hidden()
                            .text_color(cx.theme().text)
                            .bg(cx.theme().foreground)
                            .left_4()
                            .bottom(px(2.))
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
                            .border_color(cx.theme().background)
                            .children(self.ingreds.clone()),
                    )
                    // + button
                    .child(div().pt_2().h_6().w(px(self.width + 78.)).child(button(
                        "add",
                        Icon::new(IconVariant::Plus, IconSize::Small),
                        cx,
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
