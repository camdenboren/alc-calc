// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

#![windows_subsystem = "windows"]
use alc_calc::ui::{
    UI,
    util::{assets::Assets, window::window_options},
};
use gpui::{App, AppContext, Application};
use std::process;

fn main() {
    Application::new()
        .with_assets(Assets {})
        .run(|cx: &mut App| {
            cx.activate(true);
            if let Ok(_window) = cx.open_window(window_options(cx), |window, cx| {
                cx.new(|cx| UI::new(window, cx))
            }) {
            } else {
                eprintln!("alc-calc failed to open a window");
                process::exit(1)
            };
        });
}
