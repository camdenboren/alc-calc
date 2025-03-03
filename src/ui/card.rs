// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use gpui::{opaque_grey, prelude::*, Div};

pub fn card(content: Div) -> Div {
    content
        .flex()
        .flex_col()
        .size_full()
        .justify_center()
        .items_center()
        .gap_3()
        .max_w_1_2()
        .max_h_1_4()
        .bg(opaque_grey(0.2, 1.0))
        .rounded_lg()
}
