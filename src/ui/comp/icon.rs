// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ui::util::theme::ActiveTheme;
use gpui::{App, Div, Hsla, SharedString, Stateful, Window, prelude::*, svg};

#[derive(PartialEq)]
pub enum IconVariant {
    Chevron,
    Close,
    Minus,
    Plus,
    Theme,
}

impl IconVariant {
    fn path(variant: &IconVariant) -> SharedString {
        match variant {
            IconVariant::Chevron => "chevron.svg",
            IconVariant::Close => "close.svg",
            IconVariant::Minus => "minus.svg",
            IconVariant::Plus => "plus.svg",
            IconVariant::Theme => "image.svg",
        }
        .into()
    }
}

pub enum IconSize {
    Small,
    Medium,
}

impl IconSize {
    pub fn size(div: Stateful<Div>, size: &IconSize) -> Stateful<Div> {
        div.map(|this| match size {
            IconSize::Small => this.size_4(),
            IconSize::Medium => this.size_6(),
        })
    }
}

/// An Icon element with multiple variants and sizes
///
/// # Examples
///
/// ```
/// use alc_calc::ui::comp::{
///     icon::{Icon, IconSize, IconVariant},
/// };
/// use gpui::prelude::*;
///
/// struct UI {
///     icon: Icon,
/// }
///
/// impl UI {
///     fn new(cx: &mut Context<Self>) -> Self {
///         // Default color (uses `cx.theme.text()`)
///         let mut icon = Icon::new(
///             cx,
///             IconVariant::Chevron,
///             IconSize::Small,
///         );
///
///         // Specific color
///         icon = Icon::new(
///             cx,
///             IconVariant::Chevron,
///             IconSize::Small,
///         )
///             .color(gpui::black());
///         
///         UI {
///             icon,
///         }
///     }
/// }
/// ```
#[derive(IntoElement)]
pub struct Icon {
    pub variant: IconVariant,
    pub size: IconSize,
    pub color: Hsla,
    path: SharedString,
}

impl Icon {
    pub fn new(cx: &mut App, variant: IconVariant, size: IconSize) -> Self {
        let path = IconVariant::path(&variant);
        Self {
            variant,
            size,
            color: cx.theme().text,
            path,
        }
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = color;
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        svg().path(self.path).size_full().text_color(self.color)
    }
}
