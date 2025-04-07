// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod button;
pub mod card;
pub mod dropdown;
pub mod ingredient;
pub mod input;
pub mod titlebar;
use crate::calc::alc_weight;
use crate::ui::card::card;
use crate::ui::ingredient::{Ingredient, FIELDS};
use crate::ui::input::TextInput;
use crate::ui::titlebar::titlebar;
use gpui::{
    div, opaque_grey, prelude::*, px, rgb, App, Entity, FocusHandle, Focusable, Keystroke,
    SharedString, Window,
};
use std::env::consts::OS;

pub struct UI {
    text: SharedString,
    num: u32,
    num_ingredients_input: Entity<TextInput>,
    num_drinks_input: Entity<TextInput>,
    ingreds: Vec<Entity<Ingredient>>,
    pub recent_keystrokes: Vec<Keystroke>,
    focus_handle: FocusHandle,
}

impl Focusable for UI {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl UI {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let (numm, _weight) = alc_weight("Liqueur", 40.0);
        let num_ingredients_input = cx.new(|cx| TextInput::new(cx, "Type here...".into()));
        let num_drinks_input = cx.new(|cx| TextInput::new(cx, "Type here...".into()));
        cx.new(|cx| UI {
            text: "calc".into(),
            num: numm,
            num_ingredients_input,
            num_drinks_input,
            ingreds: vec![],
            recent_keystrokes: vec![],
            focus_handle: cx.focus_handle(),
        })
    }

    fn refresh(&mut self, cx: &mut App, num_ingredients: i32) {
        self.ingreds = vec![];
        for _ in 0..num_ingredients {
            self.ingreds.push(cx.new(|cx| Ingredient::new(cx)))
        }
    }
}

impl Render for UI {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let num_ingredients = self.num_ingredients_input.read(cx).content.clone();
        let num_drinks = self.num_drinks_input.read(cx).content.clone();
        let num_ingredients: i32 = match num_ingredients.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
        let num_drinks: i32 = match num_drinks.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        div()
            .font_family(".SystemUIFont")
            .bg(rgb(0x505050))
            .track_focus(&self.focus_handle(cx))
            .size_full()
            .shadow_lg()
            .text_xl()
            .text_color(rgb(0xffffff))
            .when(OS == "linux", |this| this.child(titlebar()))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .size_full()
                    .justify_center()
                    .items_center()
                    .gap_3()
                    // num_ingreds and num_parts inputs
                    .child(card(
                        div()
                            .child(format!("alc-{} {}", &self.text, &self.num))
                            .child(self.num_ingredients_input.clone())
                            .child(self.num_drinks_input.clone()),
                    ))
                    // table of ingredients
                    .when(num_ingredients > 0 && num_drinks > 0, |this| {
                        // update ingreds vec when num_ingredients changes
                        if self.ingreds.len() as i32 != num_ingredients {
                            self.refresh(cx, num_ingredients);
                        }
                        this.child(card(
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
                        ))
                    }),
            )
    }
}
