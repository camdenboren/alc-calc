// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

mod button;
mod dropdown;
mod input;
pub mod table;
mod titlebar;
use crate::ui::{table::Table, titlebar::Titlebar};
use gpui::{div, prelude::*, rgb, App, Entity, Window};
use std::env::consts::OS;

pub struct UI {
    table: Entity<Table>,
    titlebar: Entity<Titlebar>,
}

impl UI {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| UI {
            table: cx.new(|cx| Table::new(cx)),
            titlebar: cx.new(|_| Titlebar::default()),
        })
    }
}

impl Render for UI {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .font_family(".SystemUIFont")
            .bg(rgb(0x505050))
            .size_full()
            .shadow_lg()
            .text_xl()
            .text_color(rgb(0xffffff))
            .when(OS == "linux", |this| {
                this.child(self.titlebar.clone())
                    .when(!window.is_maximized(), |this| this.rounded_xl())
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .size_full()
                    .justify_center()
                    .items_center()
                    .child(self.table.clone()),
            )
    }
}
