// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::types::Type;
use crate::ui::button::*;
use gpui::{div, opaque_grey, prelude::*, SharedString, Window};
use strum::IntoEnumIterator;

pub struct Dropdown {
    current: SharedString,
    show: bool,
}

impl Dropdown {
    pub fn new() -> Self {
        Self {
            current: "Whiskey".into(),
            show: false,
        }
    }

    fn render_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .absolute()
            .top_8()
            .bg(opaque_grey(0.5, 0.5))
            .rounded_lg()
            .px_1()
            .children(Type::iter().map(|t| {
                div().child(text_button(
                    t.to_string(),
                    cx.listener(move |this, _, _window, _cx| {
                        this.update(SharedString::from(t.to_string()))
                    }),
                ))
            }))
    }

    fn toggle(&mut self) {
        if self.show {
            self.show = false;
        } else {
            self.show = true;
        }
    }

    fn update(&mut self, val: SharedString) {
        self.current = val;
        self.toggle();
    }
}

impl Render for Dropdown {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .size_full()
                    .items_center()
                    .justify_between()
                    .child(self.current.clone())
                    .child(button(
                        "",
                        "chevron.svg",
                        cx.listener(move |this, _, _window, _cx| {
                            this.toggle();
                        }),
                    )),
            )
            .when(self.show, |this| this.child(self.render_list(cx)))
    }
}
