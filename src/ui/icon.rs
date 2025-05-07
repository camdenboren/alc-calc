// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ui::theme::ActiveTheme;
use gpui::{prelude::*, svg, App, Div, SharedString, Stateful, Window};

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

#[derive(IntoElement)]
pub struct Icon {
    pub variant: IconVariant,
    pub size: IconSize,
    path: SharedString,
}

impl Icon {
    pub fn new(variant: IconVariant, size: IconSize) -> Self {
        let path = IconVariant::path(&variant);
        Self {
            variant,
            size,
            path,
        }
    }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        svg()
            .path(self.path)
            .size_full()
            .text_color(cx.theme().text)
    }
}
