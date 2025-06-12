// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

#![allow(unused_imports)]
use alc_calc::ui::{UI, assets::Assets};
use gpui::{
    App, Application, Bounds, KeyBinding, Menu, MenuItem, TitlebarOptions,
    WindowBackgroundAppearance, WindowBounds, WindowDecorations, WindowOptions, actions, px, size,
};
use std::{path::PathBuf, process};

actions!(alc_alc, [Quit]);

fn main() {
    Application::new()
        .with_assets(Assets {})
        .run(|cx: &mut App| {
            cx.activate(true);
            cx.on_action(|_: &Quit, cx| cx.quit());
            let is_linux = cfg!(target_os = "linux");
            let ctrl = if is_linux { "ctrl" } else { "cmd" };
            cx.bind_keys([KeyBinding::new(format!("{ctrl}-q").as_str(), Quit, None)]);

            cx.set_menus(vec![Menu {
                name: "alc-calc".into(),
                items: vec![MenuItem::action("Quit", Quit)],
            }]);

            if let Ok(_window) = cx.open_window(
                WindowOptions {
                    focus: true,
                    window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                        None,
                        size(px(1080.0), px(1000.0)),
                        cx,
                    ))),
                    window_decorations: Some(WindowDecorations::Client),
                    #[cfg(target_os = "macos")]
                    titlebar: Some(TitlebarOptions {
                        appears_transparent: true,
                        ..Default::default()
                    }),
                    #[cfg(target_os = "linux")]
                    window_background: WindowBackgroundAppearance::Transparent,
                    ..Default::default()
                },
                UI::new,
            ) {
            } else {
                eprintln!("alc-calc failed to open a window");
                process::exit(1)
            };
        });
}
