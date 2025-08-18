// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ui::{comp::button::text_button, util::theme::ActiveTheme};
use gpui::{
    Animation, AnimationExt, App, ElementId, FocusHandle, Focusable, SharedString, Timer, Window,
    div, prelude::*, px, svg,
};
use std::time::Duration;

pub struct Toast {
    pub description: SharedString,
    focus_handle: FocusHandle,
    dismissed: bool,
    _show: bool,
}

impl Toast {
    pub fn new(cx: &mut Context<Self>) -> Self {
        cx.spawn(async move |toast, cx| {
            Timer::after(Duration::from_secs(5)).await;
            cx.update(|cx| {
                toast.update(cx, |toast, _cx| {
                    toast.dismissed = true;
                })
            })
        })
        .detach();

        Toast {
            description: "".into(),
            focus_handle: cx.focus_handle(),
            dismissed: false,
            _show: false,
        }
    }

    pub fn toast(_cx: &mut Context<Self>, _description: &str) {
        todo!();
    }
}

impl Render for Toast {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dismissed = self.dismissed;

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
                    .child(div().text_color(cx.theme().subtext).child(
                        //self.description.clone()
                        "Longer description of error",
                    )),
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
                        this.bottom(px(delta * 75. - 30.))
                    } else {
                        this.top(px(delta * 75. - 105.))
                    }
                },
            )
    }
}

impl Focusable for Toast {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
