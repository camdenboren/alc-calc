// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

#![cfg_attr(target_family = "wasm", no_main)]
#![windows_subsystem = "windows"]
use alc_calc::ui::util::{assets::Assets, window::new_window};
use gpui::App;
use gpui_platform_gpui_unofficial::application;
#[cfg(target_family = "wasm")]
use gpui_platform_gpui_unofficial::web_init;
#[cfg(target_family = "wasm")]
use std::cell::RefCell;

#[cfg(target_family = "wasm")]
thread_local! {
    // `Application::run` consumes the `Application`, whose stack frame keeps
    // the app alive on native platforms. On web `Platform::run` invokes the
    // launch callback and returns immediately, so the app must be kept alive
    // explicitly via the handle returned by `run_embedded`.
    static APPLICATION: RefCell<Option<gpui::ApplicationHandle>> = const { RefCell::new(None) };
}

#[cfg(not(target_family = "wasm"))]
fn run() {
    application().with_assets(Assets {}).run(|cx: &mut App| {
        cx.activate(true);
        new_window(cx);
    });
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    run();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    web_init();
    let handle = application()
        .with_assets(Assets {})
        .run_embedded(|cx: &mut App| {
            cx.activate(true);
            new_window(cx);
        });
    APPLICATION.with(|cell| *cell.borrow_mut() = Some(handle));
}
