// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

#![allow(unused_imports)]
#![windows_subsystem = "windows"]
use alc_calc::ui::{UI, assets::Assets};
use gpui::{
    App, AppContext, Application, Bounds, TitlebarOptions, WindowBackgroundAppearance,
    WindowBounds, WindowDecorations, WindowOptions, px, size,
};
use std::process;

fn main() {
    Application::new()
        .with_assets(Assets {})
        .run(|cx: &mut App| {
            cx.activate(true);
            if let Ok(_window) = cx.open_window(
                WindowOptions {
                    app_id: Some("alc-calc".into()),
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
                |window, cx| cx.new(|cx| UI::new(window, cx)),
            ) {
            } else {
                eprintln!("alc-calc failed to open a window");
                process::exit(1)
            };
        });
}
