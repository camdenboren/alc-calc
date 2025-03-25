// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod card;
pub mod input;
pub mod table;
pub mod titlebar;
use crate::calc::alc_weight;
use crate::ui::card::card;
use crate::ui::input::TextInput;
use crate::ui::table::DataTable;
use crate::ui::titlebar::titlebar;
use gpui::{
    div, prelude::*, rgb, App, Entity, FocusHandle, Focusable, Keystroke, SharedString, Window,
};
use std::env::consts::OS;

pub struct UI {
    text: SharedString,
    num: u32,
    num_ingredients_input: Entity<TextInput>,
    num_drinks_input: Entity<TextInput>,
    data_table: Entity<DataTable>,
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
        let data_table = DataTable::new();
        cx.new(|cx| UI {
            text: "calc".into(),
            num: numm,
            num_ingredients_input,
            num_drinks_input,
            data_table: cx.new(|_| data_table),
            recent_keystrokes: vec![],
            focus_handle: cx.focus_handle(),
        })
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
            .child(if OS == "linux" { titlebar() } else { div() })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .size_full()
                    .justify_center()
                    .items_center()
                    .gap_3()
                    .child(card(
                        div()
                            .child(format!("alc-{} {}", &self.text, &self.num))
                            .child(self.num_ingredients_input.clone())
                            .child(self.num_drinks_input.clone()),
                    ))
                    .child(if num_ingredients > 0 && num_drinks > 0 {
                        self.data_table.update(cx, |data_table, _cx| {
                            data_table.generate(num_ingredients);
                        });
                        card(div().child(self.data_table.clone()))
                    } else {
                        div()
                    }),
            )
    }
}
