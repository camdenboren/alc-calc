// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::calc::alc_weight;
use crate::titlebar::titlebar;
use gpui::{div, prelude::*, rgb, SharedString, View, ViewContext, WindowContext};
use std::env::consts::OS;

pub struct UI {
    text: SharedString,
    num: u32,
}

impl UI {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        let (numm, _weight) = alc_weight("Liqueur", 40.0);
        cx.new_view(|_cx| UI {
            text: "calc".into(),
            num: numm,
        })
    }
}

impl Render for UI {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .font_family(".SystemUIFont")
            .bg(rgb(0x505050))
            .size_full()
            .shadow_lg()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(if OS == "linux" { titlebar() } else { div() })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .size_full()
                    .justify_center()
                    .items_center()
                    .child(format!("alc-{} {}", &self.text, &self.num)),
            )
    }
}
