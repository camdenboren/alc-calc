// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use gpui::{hsla, rgb, rgba, App, Global, Hsla, Rgba};

pub struct Theme {
    pub text: Hsla,
    pub subtext: Hsla,
    pub background: Rgba,
    pub foreground: Rgba,
    pub field: Rgba,
    pub button: Rgba,
    pub cursor: Rgba,
    pub highlight: Rgba,
}

impl Global for Theme {}

impl Theme {
    pub fn new(cx: &mut App) {
        cx.set_global::<Theme>(Theme::dark());
    }

    fn dark() -> Self {
        Self {
            text: hsla(0., 0., 0.9, 0.9),
            subtext: hsla(0., 0., 0., 0.5),
            background: rgb(0x505050),
            foreground: rgb(0x303030),
            field: rgb(0x202020),
            button: rgb(0x404040),
            cursor: rgb(0x3311ff),
            highlight: rgba(0x3311ff30),
        }
    }
}
