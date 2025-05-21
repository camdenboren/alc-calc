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
mod window_border;
use crate::ui::{
    menu::Menu,
    table::Table,
    theme::{ActiveTheme, Theme},
    titlebar::Titlebar,
    window_border::{WindowBorder, window_border},
};
use gpui::{App, Entity, Focusable, KeyBinding, Window, actions, div, prelude::*};

pub struct UI {
    menu: Entity<Menu>,
    table: Entity<Table>,
    titlebar: Entity<Titlebar>,
}

actions!(ui, [Toggle]);

const CONTEXT: &str = "UI";

impl UI {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let is_linux = cfg!(target_os = "linux");
        let ctrl = if is_linux { "ctrl" } else { "cmd" };
        cx.bind_keys([KeyBinding::new(
            format!("{ctrl}-t").as_str(),
            Toggle,
            Some(CONTEXT),
        )]);
        Theme::set(cx);
        cx.new(|cx| UI {
            menu: cx.new(|cx| Menu::new(cx)),
            table: cx.new(|cx| Table::new(cx)),
            titlebar: cx.new(|_| Titlebar::default()),
        })
    }

    fn toggle(&mut self, _: &Toggle, window: &mut Window, cx: &mut Context<Self>) {
        if self
            .table
            .read(cx)
            .focus_handle(cx)
            .contains_focused(window, cx)
        {
            self.menu.read(cx).focus(window);
            self.menu.update(cx, |menu, cx| menu.show(cx));
        } else if self.menu.read(cx).is_focused(window) {
            self.menu.update(cx, |menu, cx| menu.escape(cx));
            self.table.read(cx).num_drinks_input.read(cx).focus(window);
        }
    }
}

impl Render for UI {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let decorations = window.window_decorations();

        window_border().child(
            div()
                .key_context(CONTEXT)
                .on_action(cx.listener(Self::toggle))
                .font_family(".SystemUIFont")
                .bg(cx.theme().background)
                .size_full()
                .text_xl()
                .text_color(cx.theme().text)
                .map(|this| WindowBorder::rounding(this, decorations))
                .child(self.titlebar.clone())
                .child(self.menu.clone())
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .size_full()
                        .justify_center()
                        .items_center()
                        .child(self.table.clone()),
                ),
        )
    }
}
