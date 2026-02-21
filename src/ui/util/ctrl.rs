// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use gpui::{App, Global, SharedString};

/// The platform-specific `ctrl` key accessible across the entire app (i.e., `cmd` on
/// macOS, `ctrl` otherwise)
pub struct Ctrl {
    ctrl: SharedString,
}

impl Ctrl {
    /// Set the platform-specific `ctrl` key in the global `App` context
    ///
    /// # Examples
    /// ```
    /// use alc_calc::ui::util::ctrl::{
    ///     Ctrl,
    ///     ActiveCtrl,
    /// };
    /// use gpui::{KeyBinding, actions, prelude::*};
    ///
    /// actions!(ui, [AnAction]);
    ///
    /// struct UI {}
    ///
    /// impl UI {
    ///     fn new(cx: &mut Context<Self>) -> Self {
    ///         Ctrl::set(cx);
    ///         let ctrl = cx.ctrl();
    ///         cx.bind_keys([
    ///             KeyBinding::new(
    ///                 &format!("{ctrl}-l"),
    ///                 AnAction,
    ///                 None,
    ///             ),
    ///         ]);
    ///
    ///         UI {}
    ///     }
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// The global `Ctrl` struct will need to be initialized via `Ctrl::set(cx)` before
    /// calling `cx.ctrl()`, otherwise your application will panic
    pub fn set(cx: &mut App) {
        let is_mac = cfg!(target_os = "macos");
        let ctrl = (if is_mac { "cmd" } else { "ctrl" }).into();
        cx.set_global(Ctrl { ctrl });
    }

    fn global(cx: &App) -> SharedString {
        cx.global::<Ctrl>().ctrl.clone()
    }
}

impl Global for Ctrl {}

/// Trait for making the platform-specific `ctrl` key accessible through the `App` context
/// via `cx.ctrl()`
pub trait ActiveCtrl {
    fn ctrl(&self) -> SharedString;
}

impl ActiveCtrl for App {
    fn ctrl(&self) -> SharedString {
        Ctrl::global(self)
    }
}
