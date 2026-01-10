// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ui::{comp::button::text_button, util::theme::ActiveTheme};
use gpui::{
    Animation, AnimationExt, App, ElementId, Entity, EventEmitter, Global, SharedString, Timer,
    Window, div, prelude::*, px, svg,
};
use std::time::Duration;

pub const MAX_ITEMS: usize = 3;

pub fn toast(cx: &mut App, variant: ToastVariant, description: &str) {
    let toast = Toast::global(cx);
    toast.update(cx, |toast, cx| toast.add(cx, variant, description));
}

#[derive(Clone)]
pub enum ToastVariant {
    Info,
    Error,
}

pub struct ToastItem {
    pub description: SharedString,
    path: SharedString,
    title: SharedString,
    dismissed: bool,
    id: usize,
    count: usize,
}

impl ToastItem {
    fn new(
        cx: &mut Context<Self>,
        variant: ToastVariant,
        description: &str,
        id: usize,
        count: usize,
    ) -> Self {
        cx.spawn(async move |toast, cx| {
            Timer::after(Duration::from_secs(4)).await;
            cx.update(|cx| {
                toast.update(cx, |toast, cx| {
                    toast.dismissed = true;
                    toast.remove(cx);
                    cx.notify();
                })
            })
        })
        .detach();

        ToastItem {
            description: description.to_string().into(),
            path: ToastItem::path(&variant),
            title: ToastItem::title(&variant),
            dismissed: false,
            id,
            count,
        }
    }

    fn remove(&mut self, cx: &mut Context<Self>) {
        cx.spawn(async move |item, cx| {
            Timer::after(Duration::from_millis(200)).await;
            cx.update(|cx| {
                if let Some(item) = item.upgrade() {
                    item.update(cx, |_item, cx| cx.emit(Remove {}));
                }
            })
        })
        .detach();
    }

    fn path(variant: &ToastVariant) -> SharedString {
        match variant {
            ToastVariant::Info => "info.svg",
            ToastVariant::Error => "x_circle.svg",
        }
        .into()
    }

    fn title(variant: &ToastVariant) -> SharedString {
        match variant {
            ToastVariant::Info => "Info",
            ToastVariant::Error => "Error",
        }
        .into()
    }
}

impl Render for ToastItem {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dismissed = self.dismissed;
        let up_offset = -55. + 25. * (self.count - self.id) as f32;
        let down_offset = -80. + 15. * (self.count - self.id) as f32;

        div()
            .flex()
            .flex_row()
            .occlude()
            .absolute()
            .p_2()
            .gap_4()
            .rounded_md()
            .border_1()
            .border_color(cx.theme().border)
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
                                    .path(self.path.clone())
                                    .size_4()
                                    .text_color(cx.theme().text),
                            )
                            .child(div().text_color(cx.theme().text).child(self.title.clone())),
                    )
                    .child(
                        div()
                            .w_64()
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
                        cx.listener(move |this, _, _window, cx| {
                            this.dismissed = true;
                            this.remove(cx);
                        }),
                    )),
            )
            .with_animation(
                ElementId::NamedInteger("slide".into(), dismissed as u64),
                Animation::new(Duration::from_millis(200)),
                move |this, delta| {
                    if dismissed {
                        this.bottom(px(delta * 75. + up_offset))
                    } else {
                        this.top(px(delta * 75. + down_offset))
                    }
                },
            )
    }
}

pub struct Remove {}

impl EventEmitter<Remove> for ToastItem {}

#[derive(Default, Clone)]
pub struct Toast {
    toasts: Vec<Entity<ToastItem>>,
    count: usize,
}

impl Toast {
    fn remove(&mut self, cx: &mut Context<Self>, ix: usize) {
        if self.count > 0 && ix < self.count {
            self.toasts.remove(ix);
            self.count -= 1;
            self.toasts
                .iter()
                .for_each(|toast| toast.update(cx, |toast, _cx| toast.count = self.count));
            cx.notify();
        }
    }

    fn add(&mut self, cx: &mut Context<Self>, variant: ToastVariant, description: &str) {
        if self.count < MAX_ITEMS {
            let id = self.count;
            self.count += 1;
            let item = cx.new(|cx| ToastItem::new(cx, variant, description, id, self.count));

            cx.subscribe(
                &item,
                |this: &mut Toast, _item: Entity<ToastItem>, _event, cx| {
                    this.remove(cx, this.count - 1);
                },
            )
            .detach();
            self.toasts.push(item);
            self.toasts
                .iter()
                .for_each(|toast| toast.update(cx, |toast, _cx| toast.count = self.count));
            cx.notify();
        }
    }

    pub fn set(cx: &mut App) {
        let toast = cx.new(|_| Toast::default());
        cx.set_global(GlobalToast(toast));
    }

    pub fn global(cx: &App) -> Entity<Self> {
        cx.global::<GlobalToast>().0.clone()
    }
}

pub struct GlobalToast(Entity<Toast>);

impl Global for GlobalToast {}

impl Render for Toast {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().flex().justify_center().children(self.toasts.clone())
    }
}
