// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use alc_calc::ui::UI;
use gpui::{actions, App, AppContext, KeyBinding, Menu, MenuItem, WindowOptions};
use std::env::consts::OS;

actions!(alc_alc, [Quit]);

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        let ctrl = if OS == "linux" { "ctrl" } else { "cmd" };
        cx.bind_keys([KeyBinding::new(format!("{ctrl}-q").as_str(), Quit, None)]);

        cx.set_menus(vec![Menu {
            name: "alc-calc".into(),
            items: vec![MenuItem::action("Quit", Quit)],
        }]);

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
