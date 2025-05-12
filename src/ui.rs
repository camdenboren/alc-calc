// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod assets;
mod button;
mod dropdown;
mod icon;
mod input;
mod menu;
pub mod table;
mod theme;
mod titlebar;
use crate::ui::{
    menu::Menu,
    table::Table,
    theme::{ActiveTheme, Theme},
    titlebar::Titlebar,
};
use gpui::{App, Entity, Window, div, prelude::*, px};

pub struct UI {
    menu: Entity<Menu>,
    table: Entity<Table>,
    titlebar: Entity<Titlebar>,
}

impl UI {
    pub fn new(cx: &mut App) -> Entity<Self> {
        Theme::set(cx);
        cx.new(|cx| UI {
            menu: cx.new(|cx| Menu::new(cx)),
            table: cx.new(|cx| Table::new(cx)),
            titlebar: cx.new(|_| Titlebar::default()),
        })
    }
}

impl Render for UI {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .font_family(".SystemUIFont")
            .bg(cx.theme().background)
            .size_full()
            .shadow_lg()
            .text_xl()
            .text_color(cx.theme().text)
            .when(
                cfg!(target_os = "linux") && !window.is_maximized(),
                |this| this.border(px(0.75)).border_color(cx.theme().border),
            )
            .child(self.titlebar.clone())
            .when(!window.is_maximized(), |this| this.rounded_xl())
            .child(self.menu.clone())
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
