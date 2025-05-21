// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

// Adapted from: https://github.com/lumehq/coop/blob/master/crates/ui/src/window_border.rs

use crate::ui::theme::ActiveTheme;
use gpui::{
    AnyElement, App, Bounds, CursorStyle, Decorations, Div, Hsla, InteractiveElement as _,
    IntoElement, MouseButton, ParentElement, Pixels, Point, RenderOnce, ResizeEdge, Size, Stateful,
    Styled as _, Window, canvas, div, point, prelude::FluentBuilder as _, px,
};

const BORDER_RADIUS: Pixels = Pixels(12.0);
const BORDER_SIZE: Pixels = Pixels(0.75);
const SHADOW_SIZE: Pixels = Pixels(12.0);

pub fn window_border() -> WindowBorder {
    WindowBorder::new()
}

#[derive(IntoElement, Default)]
pub struct WindowBorder {
    children: Vec<AnyElement>,
}

#[allow(unused_variables, unreachable_code)]
impl WindowBorder {
    pub fn rounding(div: Div, decorations: Decorations) -> Div {
        if cfg!(target_os = "macos") {
            return div;
        }

        div.map(|this| match decorations {
            Decorations::Server => this,
            Decorations::Client { tiling } => this
                .when(!(tiling.top || tiling.right), |this| {
                    this.rounded_tr(BORDER_RADIUS)
                })
                .when(!(tiling.top || tiling.left), |this| {
                    this.rounded_tl(BORDER_RADIUS)
                })
                .when(!(tiling.bottom || tiling.right), |this| {
                    this.rounded_br(BORDER_RADIUS)
                })
                .when(!(tiling.bottom || tiling.left), |this| {
                    this.rounded_bl(BORDER_RADIUS)
                }),
        })
    }

    pub fn titlebar_rounding(div: Stateful<Div>, decorations: Decorations) -> Stateful<Div> {
        if cfg!(target_os = "macos") {
            return div;
        }

        div.map(|this| match decorations {
            Decorations::Server => this,
            Decorations::Client { tiling } => this
                .when(!(tiling.top || tiling.right), |this| {
                    this.rounded_tr(BORDER_RADIUS)
                })
                .when(!(tiling.top || tiling.left), |this| {
                    this.rounded_tl(BORDER_RADIUS)
                }),
        })
    }
}

impl WindowBorder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl ParentElement for WindowBorder {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for WindowBorder {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if cfg!(target_os = "macos") {
            return div()
                .id("window-border")
                .size_full()
                .children(self.children);
        }

        let decorations = window.window_decorations();
        window.set_client_inset(SHADOW_SIZE);

        div()
            .id("window-border")
            .child(
                canvas(
                    |_bounds, window, _cx| {
                        window.insert_hitbox(
                            Bounds::new(
                                point(px(0.0), px(0.0)),
                                window.window_bounds().get_bounds().size,
                            ),
                            false,
                        )
                    },
                    move |_bounds, hitbox, window, _cx| {
                        let mouse = window.mouse_position();
                        let size = window.window_bounds().get_bounds().size;
                        let Some(edge) = resize_edge(mouse, SHADOW_SIZE, size) else {
                            return;
                        };
                        window.set_cursor_style(
                            match edge {
                                ResizeEdge::Top | ResizeEdge::Bottom => CursorStyle::ResizeUpDown,
                                ResizeEdge::Left | ResizeEdge::Right => {
                                    CursorStyle::ResizeLeftRight
                                }
                                ResizeEdge::TopLeft | ResizeEdge::BottomRight => {
                                    CursorStyle::ResizeUpLeftDownRight
                                }
                                ResizeEdge::TopRight | ResizeEdge::BottomLeft => {
                                    CursorStyle::ResizeUpRightDownLeft
                                }
                            },
                            Some(&hitbox),
                        );
                    },
                )
                .size_full()
                .absolute(),
            )
            .on_mouse_move(|_e, window, _cx| window.refresh())
            .on_mouse_down(MouseButton::Left, move |_, window, _cx| {
                if window.is_maximized() {
                    return;
                }
                let size = window.window_bounds().get_bounds().size;
                let pos = window.mouse_position();

                if let Some(edge) = resize_edge(pos, SHADOW_SIZE, size) {
                    window.start_window_resize(edge)
                };
            })
            .size_full()
            .child(
                div()
                    .map(|this| WindowBorder::rounding(this, decorations))
                    .map(|div| match decorations {
                        Decorations::Server => div,
                        Decorations::Client { tiling } => div
                            .border_color(cx.theme().border)
                            .when(!tiling.top, |div| div.border_t(BORDER_SIZE))
                            .when(!tiling.bottom, |div| div.border_b(BORDER_SIZE))
                            .when(!tiling.left, |div| div.border_l(BORDER_SIZE))
                            .when(!tiling.right, |div| div.border_r(BORDER_SIZE))
                            .when(!tiling.is_tiled(), |div| {
                                div.shadow(smallvec::smallvec![gpui::BoxShadow {
                                    color: Hsla {
                                        h: 0.,
                                        s: 0.,
                                        l: 0.,
                                        a: 0.3,
                                    },
                                    blur_radius: SHADOW_SIZE / 2.,
                                    spread_radius: px(0.),
                                    offset: point(px(0.0), px(0.0)),
                                }])
                            }),
                    })
                    .on_mouse_move(|_e, window, cx| {
                        cx.stop_propagation();
                        window.refresh();
                    })
                    .size_full()
                    .children(self.children),
            )
    }
}

fn resize_edge(pos: Point<Pixels>, shadow_size: Pixels, size: Size<Pixels>) -> Option<ResizeEdge> {
    let edge = if pos.y < shadow_size && pos.x < shadow_size {
        ResizeEdge::TopLeft
    } else if pos.y < shadow_size && pos.x > size.width - shadow_size {
        ResizeEdge::TopRight
    } else if pos.y < shadow_size {
        ResizeEdge::Top
    } else if pos.y > size.height - shadow_size && pos.x < shadow_size {
        ResizeEdge::BottomLeft
    } else if pos.y > size.height - shadow_size && pos.x > size.width - shadow_size {
        ResizeEdge::BottomRight
    } else if pos.y > size.height - shadow_size {
        ResizeEdge::Bottom
    } else if pos.x < shadow_size {
        ResizeEdge::Left
    } else if pos.x > size.width - shadow_size {
        ResizeEdge::Right
    } else {
        return None;
    };
    Some(edge)
}
