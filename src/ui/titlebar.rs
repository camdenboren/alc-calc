// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ui::button::*;
use gpui::{div, prelude::*, px};

pub fn titlebar() -> impl IntoElement {
    div()
        .flex()
        .h(px(32.))
        .items_center()
        .px_4()
        .bg(gpui::opaque_grey(0.2, 1.0))
        .w_full()
        .child(div().flex().items_center().justify_center().size_full())
        .child(button("", "close.svg", |_, window, _| {
            window.remove_window();
        }))
}
