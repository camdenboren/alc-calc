// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use gpui::{div, img, prelude::*, App, SharedString, Window};

pub fn button(
    text: &str,
    icon: &str,
    on_click: impl Fn(&mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let cwd = std::env::current_dir().expect("Failed to get cwd");
    let icon_path = cwd.join("img/").join(icon);

    div()
        .id(SharedString::from(text.to_string()))
        .flex()
        .h_4()
        .w_4()
        .bg(gpui::opaque_grey(0.25, 1.0))
        .justify_center()
        .items_center()
        .active(|this| this.opacity(0.85))
        .rounded_full()
        .cursor_pointer()
        .child(img(icon_path.clone()))
        .on_click(move |_, window, cx| on_click(window, cx))
}
