// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

#![allow(unused_imports)]
use alc_calc::ui::{UI, assets::Assets};
use gpui::{
    App, Application, KeyBinding, Menu, MenuItem, TitlebarOptions, WindowBackgroundAppearance,
    WindowOptions, actions,
};
use std::path::PathBuf;

actions!(alc_alc, [Quit]);

fn main() {
    Application::new()
        .with_assets(Assets {
            base: PathBuf::from("img"),
        })
        .run(|cx: &mut App| {
            cx.activate(true);
            cx.on_action(|_: &Quit, cx| cx.quit());
            let ctrl = if cfg!(target_os = "linux") {
                "ctrl"
            } else {
                "cmd"
            };
            cx.bind_keys([KeyBinding::new(format!("{ctrl}-q").as_str(), Quit, None)]);

            cx.set_menus(vec![Menu {
                name: "alc-calc".into(),
                items: vec![MenuItem::action("Quit", Quit)],
            }]);

            cx.open_window(
                WindowOptions {
                    focus: true,
                    #[cfg(target_os = "macos")]
                    titlebar: Some(TitlebarOptions {
                        appears_transparent: true,
                        ..Default::default()
                    }),
                    #[cfg(target_os = "linux")]
                    window_background: WindowBackgroundAppearance::Transparent,
                    ..Default::default()
                },
                |_, cx| UI::new(cx),
            )
            .unwrap();
        });
}
