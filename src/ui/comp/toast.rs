// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ui::{comp::button::text_button, util::theme::ActiveTheme};
use gpui::{
    Animation, AnimationExt, App, ElementId, Entity, FocusHandle, Focusable, SharedString, Timer,
    Window, div, prelude::*, px, svg,
};
use std::time::Duration;

pub const _MAX_ITEMS: usize = 3;

pub struct ToastItem {
    pub description: SharedString,
    focus_handle: FocusHandle,
    dismissed: bool,
    id: usize,
}

impl ToastItem {
    fn new(cx: &mut Context<Self>, description: &str, id: usize) -> Self {
        cx.spawn(async move |toast, cx| {
            Timer::after(Duration::from_secs(5)).await;
            cx.update(|cx| {
                toast.update(cx, |toast, cx| {
                    toast.dismissed = true;
                    cx.notify();
                })
            })
        })
        .detach();

        ToastItem {
            description: description.to_string().into(),
            focus_handle: cx.focus_handle(),
            dismissed: false,
            id,
        }
    }
}

impl Render for ToastItem {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dismissed = self.dismissed;
        let up_offset = 30. - 15. * self.id as f32;
        let down_offset = 105. - 15. * self.id as f32;

        div()
            .flex()
            .flex_row()
            .absolute()
            .p_2()
            .gap_4()
            .rounded_md()
            .text_sm()
            .justify_center()
            .items_start()
            .bg(cx.theme().foreground)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1()
                            .child(
                                svg()
                                    .path("x_circle.svg")
                                    .size_4()
                                    .text_color(cx.theme().text),
                            )
                            .child(div().text_color(cx.theme().text).child("Error")),
                    )
                    .child(
                        div()
                            .text_color(cx.theme().subtext)
                            .child(self.description.clone()),
                    ),
            )
            .child(
                div()
                    .rounded_md()
                    .py_1()
                    .px_2()
                    .bg(cx.theme().field)
                    .child(text_button(
                        "okay",
                        "Okay".into(),
                        cx.listener(move |this, _, _window, _cx| {
                            println!("Clicked Okay");
                            this.dismissed = true;
                        }),
                    )),
            )
            .with_animation(
                ElementId::NamedInteger("slide".into(), dismissed as u64),
                Animation::new(Duration::from_millis(200)),
                move |this, delta| {
                    if dismissed {
                        this.bottom(px(delta * 75. - up_offset))
                    } else {
                        this.top(px(delta * 75. - down_offset))
                    }
                },
            )
    }
}

impl Focusable for ToastItem {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

pub struct Toast {
    toasts: Vec<Entity<ToastItem>>,
    _count: usize,
}

impl Toast {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Toast {
            toasts: vec![
                cx.new(|cx| ToastItem::new(cx, "Longer description of first", 0)),
                cx.new(|cx| ToastItem::new(cx, "Longer description of second", 1)),
            ],
            _count: 2,
        }
    }

    pub fn _toast(&mut self, cx: &mut Context<Self>, description: &str) {
        if self._count < _MAX_ITEMS {
            let id = self._count;
            let item = cx.new(|cx| ToastItem::new(cx, description, id));
            self.toasts.push(item);
            self._count += 1;
        }
        cx.notify();
    }
}

impl Render for Toast {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().flex().justify_center().children(self.toasts.clone())
    }
}
