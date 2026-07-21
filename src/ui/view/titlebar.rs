// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

// Move-by-mouse from: https://github.com/zed-industries/zed/blob/main/crates/title_bar/src/title_bar.rs

#![cfg(not(target_os = "windows"))]

#[cfg(target_os = "linux")]
use crate::ui::comp::{
    button::icon_button,
    icon::{Icon, IconSize, IconVariant},
};
use crate::ui::util::{theme::ActiveTheme, window::WindowBorder};
#[cfg(not(target_os = "linux"))]
use gpui::Empty;
use gpui::{Window, div, prelude::*, px};

#[cfg(target_os = "linux")]
const HEIGHT: f32 = 36.;
#[cfg(not(target_os = "linux"))]
const HEIGHT: f32 = 28.;

/// A simple `Titlebar` element used on Linux and macOS which handles mouse-driven
/// - Window zoom toggling
/// - Window-activation-specific colors
/// - Window movement and it's associated state (e.g., `should_move`)
/// - Application quitting (via the Gnome-inspired quit `Icon`) on Linux
///
/// This entire module is ignored on Windows targets as we instead rely on GPUI's default
/// titlebar for that target instead
#[derive(Default)]
pub struct Titlebar {
    should_move: bool,
}

impl Titlebar {
    /// Create a `Titlebar` element
    ///
    /// # Examples
    ///
    /// ```
    /// use alc_calc::ui::view::titlebar::Titlebar;
    /// use gpui::{Entity, prelude::*};
    ///
    /// struct UI {
    ///     titlebar: Entity<Titlebar>,
    /// }
    ///
    /// impl UI {
    ///     fn new(cx: &mut Context<Self>) -> Self {
    ///         UI {
    ///             titlebar: cx.new(|_| Titlebar::new())
    ///         }
    ///     }
    /// }
    /// ```
    pub fn new() -> Self {
        Titlebar::default()
    }
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
                true => this.bg(cx.theme().titlebar),
                false => this.bg(cx.theme().titlebar_inactive),
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
                        .child(icon_button(
                            "quit",
                            Icon::new(cx, IconVariant::Close, IconSize::Medium),
                            |_, window, _| {
                                window.remove_window();
                            },
                        ))
                        .block_mouse_except_scroll()
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
