// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use alc_calc::ui::UI;
use gpui::{
    actions, App, Application, KeyBinding, Menu, MenuItem, WindowBackgroundAppearance,
    WindowOptions,
};
use std::env::consts::OS;

actions!(alc_alc, [Quit]);

fn main() {
    Application::new().run(|cx: &mut App| {
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
                window_background: WindowBackgroundAppearance::Transparent,
                ..Default::default()
            },
            |_, cx| UI::new(cx),
        )
        .unwrap();
    });
}
