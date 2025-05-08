// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    types::Type,
    ui::{
        button::button,
        button::text_button,
        icon::{Icon, IconSize, IconVariant},
        table::MAX_ITEMS,
        theme::ActiveTheme,
    },
};
use gpui::{
    App, FocusHandle, Focusable, KeyBinding, ScrollStrategy, SharedString, UniformListScrollHandle,
    Window, actions, deferred, div, prelude::*, px, uniform_list,
};
use std::cmp::max;
use strum::{EnumCount, IntoEnumIterator};

actions!(dropdown, [Escape, Enter, Next, Prev, Select]);

const CONTEXT: &str = "Dropdown";

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
            .key_context(CONTEXT)
            .flex()
            .flex_col()
            .absolute()
            .top_9()
            .right(px(0.))
            .bg(cx.theme().field)
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
                                    .hover(|this| this.bg(cx.theme().background))
                                    .when(this.focused_item == ix as isize, |this| {
                                        this.bg(cx.theme().background)
                                    })
                                    .child(text_button(
                                        format!("dropdown_item_{ix}").as_str(),
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
                    cx.stop_propagation();
                    this.escape(&Escape, window, cx);
                }))
                .h_full(),
            )
    }

    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        cx.stop_propagation();
        self.show = !self.show;
    }

    fn update(&mut self, window: &mut Window, cx: &mut Context<Self>, val: SharedString) {
        self.focused_item = self.types.iter().position(|t| *t == val).unwrap() as isize;
        self.current = val;
        self.toggle(cx);
        self.focus_handle.focus(window);
    }

    fn escape(&mut self, _: &Escape, _window: &mut Window, cx: &mut Context<Self>) {
        self.show = false;
        cx.notify();
    }

    fn show(&mut self, _: &Enter, _window: &mut Window, cx: &mut Context<Self>) {
        self.show = true;
        cx.notify();
    }

    fn select(&mut self, _: &Select, window: &mut Window, cx: &mut Context<Self>) {
        self.update(
            window,
            cx,
            self.types[max(self.focused_item, 0) as usize].clone(),
        );
        cx.notify();
    }

    fn next(&mut self, _: &Next, _window: &mut Window, cx: &mut Context<Self>) {
        if self.focused_item < (self.count - 1) as isize {
            self.focused_item += 1;
        } else {
            self.focused_item = 0;
        }
        self.scroll();
        cx.notify();
    }

    fn prev(&mut self, _: &Prev, _window: &mut Window, cx: &mut Context<Self>) {
        if self.focused_item <= 0 {
            self.focused_item = (self.count - 1) as isize;
        } else {
            self.focused_item -= 1;
        }
        self.scroll();
        cx.notify();
    }

    fn scroll(&mut self) {
        self.scroll_handle
            .scroll_to_item(self.focused_item as usize, ScrollStrategy::Top);
    }
}

impl Render for Dropdown {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        deferred(
            div()
                .flex()
                .flex_col()
                .key_context(CONTEXT)
                .when(self.show, |this| {
                    this.on_action(cx.listener(Self::escape))
                        .on_action(cx.listener(Self::select))
                        .on_action(cx.listener(Self::next))
                        .on_action(cx.listener(Self::prev))
                })
                .when(!self.show, |this| this.on_action(cx.listener(Self::show)))
                .track_focus(&self.focus_handle)
                .bg(cx.theme().field)
                .border_1()
                .border_color(cx.theme().field)
                .focus(|this| this.border_color(cx.theme().cursor))
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
                            "dropdown",
                            Icon::new(IconVariant::Chevron, IconSize::Small),
                            cx,
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
