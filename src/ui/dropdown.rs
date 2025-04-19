// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::types::Type;
use crate::ui::button::*;
use crate::ui::table::MAX_ITEMS;
use gpui::{
    actions, deferred, div, opaque_grey, prelude::*, px, uniform_list, App, FocusHandle, Focusable,
    KeyBinding, SharedString, Window,
};
use strum::{EnumCount, IntoEnumIterator};

actions!(dropdown, [Escape,]);

pub struct Dropdown {
    pub current: SharedString,
    show: bool,
    id: usize,
    focus_handle: FocusHandle,
}

impl Dropdown {
    pub fn new(id: usize, cx: &mut App) -> Self {
        Self {
            current: "Whiskey".into(),
            show: false,
            id,
            focus_handle: cx.focus_handle(),
        }
    }

    fn render_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .absolute()
            .top_12()
            .right(px(0.))
            .bg(opaque_grey(0.5, 0.5))
            .rounded_md()
            .p_1()
            .w_full()
            .h_80()
            .child(
                uniform_list(
                    cx.entity().clone(),
                    "ingreds_list",
                    Type::COUNT,
                    |_this, range, _window, cx| {
                        let mut items = Vec::new();
                        let types: Vec<SharedString> = Type::iter()
                            .map(|t| SharedString::from(t.to_string()))
                            .collect();

                        for ix in range {
                            let item = types[ix].clone();
                            items.push(
                                div()
                                    .rounded_md()
                                    .px_1()
                                    .hover(|this| this.bg(opaque_grey(0.7, 0.5)))
                                    .child(text_button(
                                        item.clone(),
                                        cx.listener(move |this, _, _window, _cx| {
                                            this.update(item.clone());
                                        }),
                                    )),
                            );
                        }
                        items
                    },
                )
                .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                    this.escape(&Escape, window, cx);
                }))
                .h_full(),
            )
    }

    fn toggle(&mut self) {
        self.show = !self.show;
    }

    fn update(&mut self, val: SharedString) {
        self.current = val;
        self.toggle();
    }

    fn escape(&mut self, _: &Escape, _window: &mut Window, _cx: &mut Context<Self>) {
        self.show = false;
    }
}

impl Render for Dropdown {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        cx.bind_keys([KeyBinding::new("escape", Escape, None)]);

        deferred(
            div()
                .flex()
                .flex_col()
                .key_context("Dropdown")
                .on_action(cx.listener(Self::escape))
                .track_focus(&self.focus_handle)
                .bg(opaque_grey(0.1, 0.5))
                .px_2()
                .py_1()
                .rounded_md()
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
                .when(self.show, |this| this.child(self.render_list(cx))),
        )
        .with_priority(MAX_ITEMS - self.id)
    }
}

impl Focusable for Dropdown {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
