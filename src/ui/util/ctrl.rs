// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use gpui::{App, Global, SharedString};

pub struct Ctrl {
    ctrl: SharedString,
}

impl Ctrl {
    pub fn set(cx: &mut App) {
        let is_mac = cfg!(target_os = "macos");
        let ctrl = (if is_mac { "cmd" } else { "ctrl" }).into();
        cx.set_global(Ctrl { ctrl });
    }

    pub fn global(cx: &App) -> SharedString {
        cx.global::<Ctrl>().ctrl.clone()
    }
}

impl Global for Ctrl {}

pub trait ActiveCtrl {
    fn ctrl(&self) -> SharedString;
}
