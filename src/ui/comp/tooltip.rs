// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ui::util::theme::ActiveTheme;
use gpui::{AnyView, App, SharedString, Window, div, prelude::*};

#[derive(Default)]
pub struct Tooltip {
    text: SharedString,
    keybind: Option<SharedString>,
}

impl Tooltip {
    pub fn new(text: SharedString, keybind: Option<SharedString>) -> Self {
        Self {
            text: text.clone(),
            keybind: keybind,
        }
    }

    pub fn build(self, _window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|_| self).into()
    }
}

impl FluentBuilder for Tooltip {}

impl Render for Tooltip {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .justify_center()
            .p_2()
            .gap_4()
            .border_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().foreground)
            .text_color(cx.theme().text)
            .text_sm()
            .rounded_md()
            .child(self.text.clone())
            .when_some(self.keybind.clone(), |this, _| {
                this.child(
                    div()
                        .flex()
                        .text_color(cx.theme().subtext)
                        .child(self.keybind.clone().unwrap()),
                )
            })
    }
}
