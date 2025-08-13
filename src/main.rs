// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

#![windows_subsystem = "windows"]
use alc_calc::ui::util::{assets::Assets, window::new_window};
use gpui::{App, Application};

fn main() {
    Application::new()
        .with_assets(Assets {})
        .run(|cx: &mut App| {
            cx.activate(true);
            new_window(cx);
        });
}
