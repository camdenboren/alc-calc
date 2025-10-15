// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ui::comp::icon::{Icon, IconSize, IconVariant};
use gpui::{App, ClickEvent, SharedString, Window, div, prelude::*, px};

pub fn button(
    id: &str,
    text: SharedString,
    icon: Icon,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id.to_string().into_element())
        .flex()
        .flex_row()
        .size_full()
        .items_center()
        .justify_between()
        .h(px(20. + 4. * 2.))
        .active(|this| this.opacity(0.85))
        .cursor_pointer()
        .child(text)
        .child(
            div()
                .id(format!("{id}-icon").into_element())
                .map(|this| IconSize::size(this, &icon.size))
                .child(icon),
        )
        .on_click(move |event, window, cx| on_click(event, window, cx))
}

pub fn icon_button(
    id: &str,
    icon: Icon,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id.to_string().into_element())
        .flex()
        .map(|this| IconSize::size(this, &icon.size))
        .justify_center()
        .items_center()
        .active(|this| this.opacity(0.85))
        .when(icon.variant != IconVariant::Close, |this| {
            this.cursor_pointer()
        })
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
