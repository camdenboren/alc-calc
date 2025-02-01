// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use alc_calc::ui::UI;
use gpui::{App, AppContext, WindowOptions};

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(
            WindowOptions {
                focus: true,
                ..Default::default()
            },
            |cx| UI::new(cx),
        )
        .unwrap();
    });
}
