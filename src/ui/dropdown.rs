// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::types::Type;
use crate::ui::button::*;
use gpui::{div, opaque_grey, prelude::*, Div, SharedString, Window};
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

    fn render_list(&self) -> Div {
        div()
            .flex()
            .flex_col()
            .size_full()
            .absolute()
            .bg(opaque_grey(0.5, 0.5))
            .rounded_lg()
            .children(Type::iter().map(|t| t.to_string()))
    }

    fn toggle(&mut self) {
        if self.show {
            self.show = false;
        } else {
            self.show = true;
        }
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
                    .gap_x_1()
                    .child(self.current.clone())
                    .child(button(
                        "",
                        "chevron.svg",
                        cx.listener(move |this, _, _window, _cx| {
                            this.toggle();
                        }),
                    )),
            )
            .child(if self.show { self.render_list() } else { div() })
    }
}
