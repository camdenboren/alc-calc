// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

// Container padding from: https://github.com/zed-industries/zed/blob/main/crates/ui/src/components/tooltip.rs

use crate::ui::util::theme::ActiveTheme;
use gpui::{SharedString, Window, div, prelude::*};

/// A Tooltip element that can display text and an optional keybind
///
/// The Tooltip will need to be converted to an `AnyView` before passed to `.tooltip()`,
/// which is only available when the element is a `Stateful<Div>` (hence the `id()`)
///
/// # Examples
///
/// ```
/// use alc_calc::ui::comp::tooltip::Tooltip;
/// use gpui::{div, prelude::*};
///
/// // Basic Tooltip
/// div()
///     .id("id".into_element())
///     .tooltip(|_window, cx| {
///         cx.new(|_cx| Tooltip::new("Text")).into()
///     });
///
/// // Tooltip with keybind
/// div()
///     .id("id".into_element())
///     .tooltip(|_window, cx| {
///         cx.new(|_cx| {
///             Tooltip::new("Text").keybind("Keybind")
///         }).into()
///     });
/// ```
#[derive(Default)]
pub struct Tooltip {
    text: SharedString,
    keybind: Option<SharedString>,
}

impl Tooltip {
    /// Create a Tooltip with text
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string().into(),
            keybind: None,
        }
    }

    /// Add a keybind to the Tooltip
    pub fn keybind(mut self, keybind: &str) -> Self {
        self.keybind = Some(keybind.to_string().into());
        self
    }
}

impl FluentBuilder for Tooltip {}

impl Render for Tooltip {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // padding to avoid tooltip appearing right below the mouse cursor
        div().pl_2().pt_2p5().child(
            div()
                .flex()
                .flex_row()
                .justify_center()
                .p_2()
                .gap_4()
                .border_1()
                .border_color(cx.theme().border)
                .bg(cx.theme().foreground)
                .text_color(cx.theme().text)
                .text_sm()
                .rounded_md()
                .child(self.text.clone())
                .when_some(self.keybind.clone(), |this, _| {
                    this.child(
                        div()
                            .flex()
                            .text_color(cx.theme().subtext)
                            .child(self.keybind.clone().unwrap()),
                    )
                }),
        )
    }
}
