// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

// move-by-mouse from zed's titlebar: title_bar/src/title_bar.rs

use crate::ui::button::*;
use gpui::{div, prelude::*, px, Window};

pub struct Titlebar {
    should_move: bool,
}

impl Titlebar {
    pub fn new() -> Self {
        Self { should_move: false }
    }
}

impl Render for Titlebar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("titlebar")
            .flex()
            .h(px(32.))
            .items_center()
            .px_4()
            .bg(gpui::opaque_grey(0.2, 1.0))
            .w_full()
            .on_click(|event, window, _| {
                if event.up.click_count == 2 {
                    window.zoom_window();
                }
            })
            .on_mouse_move(cx.listener(move |this, _ev, window, _| {
                if this.should_move {
                    this.should_move = false;
                    window.start_window_move();
                }
            }))
            .on_mouse_down_out(cx.listener(move |this, _ev, _window, _cx| {
                this.should_move = false;
            }))
            .on_mouse_up(
                gpui::MouseButton::Left,
                cx.listener(move |this, _ev, _window, _cx| {
                    this.should_move = false;
                }),
            )
            .on_mouse_down(
                gpui::MouseButton::Left,
                cx.listener(move |this, _ev, _window, _cx| {
                    this.should_move = true;
                }),
            )
            .child(div().flex().items_center().justify_center().size_full())
            .child(button("", "close.svg", |_, window, _| {
                window.remove_window();
            }))
    }
}
