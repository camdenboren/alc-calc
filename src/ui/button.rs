// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ui::{
    icon::{Icon, IconSize},
    theme::ActiveTheme,
};
use gpui::{App, ClickEvent, SharedString, Window, div, prelude::*};

use super::icon::IconVariant;

pub fn button(
    id: &str,
    icon: Icon,
    cx: &App,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id.to_string().into_element())
        .flex()
        .map(|this| IconSize::size(this, &icon.size))
        .bg(cx.theme().button)
        .justify_center()
        .items_center()
        .active(|this| this.opacity(0.85))
        .rounded_full()
        .when(icon.variant == IconVariant::Theme, |this| this.rounded_md())
        .cursor_pointer()
        .child(icon)
        .on_click(move |event, window, cx| on_click(event, window, cx))
}

pub fn text_button(
    id: &str,
    text: SharedString,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id.to_string().into_element())
        .flex()
        .active(|this| this.opacity(0.85))
        .cursor_pointer()
        .child(text)
        .on_click(move |event, window, cx| on_click(event, window, cx))
}
