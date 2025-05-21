// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ui::{
    button::{button, text_button},
    icon::{Icon, IconSize, IconVariant},
    theme::{ActiveTheme, Theme, ThemeVariant},
};
use gpui::{
    App, FocusHandle, Focusable, KeyBinding, SharedString, Window, actions, div, prelude::*, px,
    uniform_list,
};
use std::cmp::max;
use strum::{EnumCount, IntoEnumIterator};

actions!(menu, [Escape, Enter, Next, Prev, Select]);

const CONTEXT: &str = "Menu";

pub struct Menu {
    variants: Vec<SharedString>,
    show: bool,
    count: usize,
    focused_item: isize,
    focus_handle: FocusHandle,
}

impl Menu {
    pub fn new(cx: &mut App) -> Self {
        cx.bind_keys([
            KeyBinding::new("escape", Escape, Some(CONTEXT)),
            KeyBinding::new("enter", Enter, Some(CONTEXT)),
            KeyBinding::new("up", Prev, Some(CONTEXT)),
            KeyBinding::new("k", Prev, Some(CONTEXT)),
            KeyBinding::new("down", Next, Some(CONTEXT)),
            KeyBinding::new("j", Next, Some(CONTEXT)),
            KeyBinding::new("enter", Select, Some(CONTEXT)),
        ]);

        Self {
            variants: ThemeVariant::iter()
                .map(|t| SharedString::from(t.to_string()))
                .collect(),
            show: false,
            count: ThemeVariant::COUNT,
            focused_item: -1,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn focus(&self, window: &mut Window) {
        self.focus_handle.focus(window)
    }

    pub fn is_focused(&self, window: &mut Window) -> bool {
        self.focus_handle.is_focused(window)
    }

    fn update(&mut self, val: SharedString, cx: &mut Context<Self>) {
        self.focused_item = self.variants.iter().position(|t| *t == val).unwrap() as isize;
        Theme::update(&val, cx);
        self.toggle();
    }

    pub fn toggle(&mut self) {
        self.show = !self.show;
    }

    pub fn escape(&mut self, cx: &mut Context<Self>) {
        self.show = false;
        cx.notify();
    }

    pub fn show(&mut self, cx: &mut Context<Self>) {
        self.show = true;
        cx.notify();
    }

    fn escape_key(&mut self, _: &Escape, _window: &mut Window, cx: &mut Context<Self>) {
        self.show = false;
        cx.notify();
    }

    fn show_key(&mut self, _: &Enter, _window: &mut Window, cx: &mut Context<Self>) {
        self.show = true;
        cx.notify();
    }

    fn select(&mut self, _: &Select, _window: &mut Window, cx: &mut Context<Self>) {
        self.update(
            self.variants[max(self.focused_item, 0) as usize].clone(),
            cx,
        );
        cx.notify();
    }

    fn next(&mut self, _: &Next, _window: &mut Window, cx: &mut Context<Self>) {
        if self.focused_item < (self.count - 1) as isize {
            self.focused_item += 1;
        } else {
            self.focused_item = 0;
        }
        cx.notify();
    }

    fn prev(&mut self, _: &Prev, _window: &mut Window, cx: &mut Context<Self>) {
        if self.focused_item <= 0 {
            self.focused_item = (self.count - 1) as isize;
        } else {
            self.focused_item -= 1;
        }
        cx.notify();
    }
}

impl Render for Menu {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .key_context(CONTEXT)
            .when(self.show, |this| {
                this.on_action(cx.listener(Self::escape_key))
                    .on_action(cx.listener(Self::select))
                    .on_action(cx.listener(Self::next))
                    .on_action(cx.listener(Self::prev))
            })
            .when(!self.show, |this| {
                this.on_action(cx.listener(Self::show_key))
            })
            .track_focus(&self.focus_handle)
            .justify_start()
            .items_end()
            .p_2()
            .child(button(
                "menu",
                Icon::new(IconVariant::Theme, IconSize::Medium),
                cx,
                cx.listener(move |this, _, _, _| this.toggle()),
            ))
            .when(self.show, |this| {
                this.child(
                    div()
                        .flex()
                        .flex_col()
                        .absolute()
                        .top_10()
                        .w_40()
                        .h(px(168.))
                        .bg(cx.theme().field)
                        .rounded_md()
                        .p_1()
                        .child(
                            uniform_list(
                                cx.entity(),
                                "themes_list",
                                self.count,
                                |this, range, _window, cx| {
                                    range
                                        .map(|ix| {
                                            let item = this.variants[ix].clone();
                                            div()
                                                .rounded_md()
                                                .px_1()
                                                .hover(|this| this.bg(cx.theme().background))
                                                .when(this.focused_item == ix as isize, |this| {
                                                    this.bg(cx.theme().background)
                                                })
                                                .child(text_button(
                                                    format!("theme_item_{ix}").as_str(),
                                                    item.clone(),
                                                    cx.listener(move |this, _, _window, cx| {
                                                        this.update(item.clone(), cx);
                                                    }),
                                                ))
                                        })
                                        .collect()
                                },
                            )
                            .on_mouse_down_out(cx.listener(|this, _, _window, cx| {
                                cx.stop_propagation();
                                cx.notify();
                                this.toggle();
                            }))
                            .h_full(),
                        ),
                )
            })
    }
}

impl Focusable for Menu {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
