// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ui::button::*;
use gpui::{div, opaque_grey, prelude::*, Div, SharedString, Window};

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
            .justify_center()
            .items_center()
            .gap_3()
            .max_w_1_12()
            .max_h_1_4()
            .bg(opaque_grey(0.2, 1.0))
            .rounded_lg()
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
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_x_1()
            .child(self.current.clone())
            .child(button("", "chevron.svg", |_, _| {
                println!("Clicked Dropdown Button");
            }))
            .child(if self.show { self.render_list() } else { div() })
    }
}
