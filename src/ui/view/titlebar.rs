// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

// Move-by-mouse from: https://github.com/zed-industries/zed/blob/main/crates/title_bar/src/title_bar.rs

#[cfg(target_os = "linux")]
use crate::ui::comp::{
    button::button,
    icon::{Icon, IconSize, IconVariant},
};
use crate::ui::util::{theme::ActiveTheme, window::WindowBorder};
use gpui::{Empty, Window, div, prelude::*, px};

#[cfg(target_os = "linux")]
const HEIGHT: f32 = 36.;
#[cfg(not(target_os = "linux"))]
const HEIGHT: f32 = 28.;

#[derive(Default)]
pub struct Titlebar {
    should_move: bool,
}

impl Render for Titlebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("titlebar")
            .flex()
            .h(px(HEIGHT))
            .w_full()
            .border_b(px(0.5))
            .border_color(cx.theme().separator)
            .map(|this| WindowBorder::titlebar_rounding(this, window.window_decorations()))
            .map(|this| match window.is_window_active() {
                true => this.bg(cx.theme().foreground),
                false => this.bg(cx.theme().foreground_inactive),
            })
            .items_center()
            .justify_end()
            .px_2()
            .on_click(|event, window, _| {
                if event.click_count() == 2 {
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
            .when(cfg!(target_os = "linux"), |this| {
                this.child(
                    #[cfg(not(target_os = "linux"))]
                    Empty,
                    #[cfg(target_os = "linux")]
                    div()
                        .id("quit-div")
                        .child(button(
                            "quit",
                            Icon::new(IconVariant::Close, IconSize::Medium),
                            |_, window, _| {
                                window.remove_window();
                            },
                        ))
                        .map(|this| match window.is_window_active() {
                            true => this.bg(cx.theme().close_button),
                            false => this.bg(cx.theme().close_button_inactive),
                        })
                        .hover(|this| this.bg(cx.theme().close_button_hover))
                        .active(|this| this.bg(cx.theme().close_button_click))
                        .rounded_full(),
                )
            })
    }
}
