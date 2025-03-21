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
    div, prelude::*, rgb, App, Entity, FocusHandle, Focusable, Keystroke, SharedString,
    UniformListScrollHandle, Window,
};
use std::env::consts::OS;

pub struct UI {
    text: SharedString,
    num: u32,
    text_input: Entity<TextInput>,
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
        let text_input = cx.new(|cx| TextInput {
            focus_handle: cx.focus_handle(),
            content: "".into(),
            placeholder: "Type here...".into(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            is_selecting: false,
        });
        let mut data_table = DataTable {
            ingreds: Vec::new(),
            visible_range: 0..0,
            scroll_handle: UniformListScrollHandle::new(),
            drag_position: None,
        };
        data_table.generate();
        cx.new(|cx| UI {
            text: "calc".into(),
            num: numm,
            text_input,
            data_table: cx.new(|_| data_table),
            recent_keystrokes: vec![],
            focus_handle: cx.focus_handle(),
        })
    }
}

impl Render for UI {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let num_ingredients = self.text_input.read(cx).content.clone();
        let num_ingredients: i32 = match num_ingredients.trim().parse() {
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
                            .child(self.text_input.clone()),
                    ))
                    .child(if num_ingredients > 0 {
                        card(div().child(self.data_table.clone()))
                    } else {
                        div()
                    }),
            )
    }
}
