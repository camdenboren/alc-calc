// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    types::Type,
    ui::{button::button, button::text_button, table::MAX_ITEMS},
};
use gpui::{
    actions, deferred, div, opaque_grey, prelude::*, px, uniform_list, App, FocusHandle, Focusable,
    Global, KeyBinding, ScrollStrategy, SharedString, UniformListScrollHandle, Window,
};
use std::cmp::max;
use std::time::Duration;
use strum::{EnumCount, IntoEnumIterator};

actions!(dropdown, [Escape, Enter, Next, Prev, Select]);

#[derive(Default)]
pub struct DropdownState {
    pub just_clicked: bool,
    pub delayed: bool,
    pub count: usize,
}

impl Global for DropdownState {}

pub struct Dropdown {
    types: Vec<SharedString>,
    pub current: SharedString,
    pub show: bool,
    count: usize,
    pub id: usize,
    focused_item: isize,
    focus_handle: FocusHandle,
    scroll_handle: UniformListScrollHandle,
}

impl Dropdown {
    pub fn new(id: usize, cx: &mut App) -> Self {
        // hack to allow hiding dropdown when clicking toggle by preventing
        // on_mouse_down_out and toggle from executing together
        //
        // todo: mv state to dropdown to avoid unneeded global scope and allow
        // toggling another dropdown on_mouse_down_out
        if id == 0 {
            cx.set_global::<DropdownState>(DropdownState::default());
            cx.spawn(|cx| async move {
                loop {
                    cx.update_global::<DropdownState, _>(|state, _cx| {
                        if state.delayed {
                            state.count += 1;
                            if state.count >= 2 {
                                state.delayed = false;
                            }
                        } else {
                            state.count = 0;
                        }

                        if state.just_clicked {
                            state.delayed = true;
                            state.just_clicked = false;
                        }
                    })
                    .expect("Unexpectedly failed to update global dropdown state");

                    cx.background_executor()
                        .timer(Duration::from_millis(50))
                        .await;
                }
            })
            .detach();
        }

        Self {
            types: Type::iter()
                .map(|t| SharedString::from(t.to_string()))
                .collect(),
            current: "Whiskey".into(),
            show: false,
            count: Type::COUNT,
            id,
            focused_item: -1,
            focus_handle: cx.focus_handle(),
            scroll_handle: UniformListScrollHandle::new(),
        }
    }

    pub fn focus(&self, window: &mut Window) {
        self.focus_handle.focus(window)
    }

    pub fn is_focused(&self, window: &mut Window) -> bool {
        self.focus_handle.is_focused(window)
    }

    fn render_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .absolute()
            .top_9()
            .right(px(0.))
            .bg(opaque_grey(0.15, 1.0))
            .rounded_md()
            .p_1()
            .w_full()
            .h_80()
            .child(
                uniform_list(
                    cx.entity(),
                    "ingreds_list",
                    self.count,
                    |this, range, _window, cx| {
                        range
                            .map(|ix| {
                                let item = this.types[ix].clone();
                                div()
                                    .rounded_md()
                                    .px_1()
                                    .hover(|this| this.bg(opaque_grey(0.7, 0.5)))
                                    .when(this.focused_item == ix as isize, |this| {
                                        this.bg(opaque_grey(0.7, 0.5))
                                    })
                                    .child(text_button(
                                        item.clone(),
                                        cx.listener(move |this, _, window, cx| {
                                            this.update(window, cx, item.clone());
                                        }),
                                    ))
                            })
                            .collect()
                    },
                )
                .track_scroll(self.scroll_handle.clone())
                .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                    cx.global_mut::<DropdownState>().just_clicked = true;
                    this.escape(&Escape, window, cx);
                }))
                .h_full(),
            )
    }

    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        if !cx.global_mut::<DropdownState>().delayed {
            cx.stop_propagation();
            self.show = !self.show;
        }
    }

    fn update(&mut self, window: &mut Window, cx: &mut Context<Self>, val: SharedString) {
        self.focused_item = self.types.iter().position(|t| *t == val).unwrap() as isize;
        self.current = val;
        self.toggle(cx);
        self.focus_handle.focus(window);
    }

    fn escape(&mut self, _: &Escape, _window: &mut Window, _cx: &mut Context<Self>) {
        self.show = false;
    }

    fn show(&mut self, _: &Enter, _window: &mut Window, _cx: &mut Context<Self>) {
        self.show = true;
    }

    fn select(&mut self, _: &Select, window: &mut Window, cx: &mut Context<Self>) {
        self.update(
            window,
            cx,
            self.types[max(self.focused_item, 0) as usize].clone(),
        )
    }

    fn next(&mut self, _: &Next, _window: &mut Window, _cx: &mut Context<Self>) {
        if self.focused_item < (self.count - 1) as isize {
            self.focused_item += 1;
        } else {
            self.focused_item = 0;
        }
        self.scroll();
    }

    fn prev(&mut self, _: &Prev, _window: &mut Window, _cx: &mut Context<Self>) {
        if self.focused_item <= 0 {
            self.focused_item = (self.count - 1) as isize;
        } else {
            self.focused_item -= 1;
        }
        self.scroll();
    }

    fn scroll(&mut self) {
        self.scroll_handle
            .scroll_to_item(self.focused_item as usize, ScrollStrategy::Top);
    }
}

impl Render for Dropdown {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        cx.bind_keys([
            KeyBinding::new("escape", Escape, None),
            KeyBinding::new("enter", Enter, None),
            KeyBinding::new("up", Prev, None),
            KeyBinding::new("k", Prev, None),
            KeyBinding::new("down", Next, None),
            KeyBinding::new("j", Next, None),
            KeyBinding::new("enter", Select, None),
        ]);

        deferred(
            div()
                .flex()
                .flex_col()
                .key_context("Dropdown")
                .when(self.show, |this| {
                    this.on_action(cx.listener(Self::escape))
                        .on_action(cx.listener(Self::select))
                        .on_action(cx.listener(Self::next))
                        .on_action(cx.listener(Self::prev))
                })
                .when(!self.show, |this| this.on_action(cx.listener(Self::show)))
                .track_focus(&self.focus_handle)
                .bg(opaque_grey(0.15, 1.0))
                .border_1()
                .border_color(opaque_grey(0.15, 1.0))
                .focus(|this| this.border_color(gpui::Hsla::blue()))
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
                        .h(px(20. + 4. * 2.))
                        .child(self.current.clone())
                        .child(button(
                            "",
                            "chevron.svg",
                            cx.listener(move |this, _, _window, cx| {
                                this.toggle(cx);
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
