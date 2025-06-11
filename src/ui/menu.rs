// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ui::{
    button::{button, text_button},
    icon::{Icon, IconSize, IconVariant},
    theme::{ActiveTheme, Theme, ThemeVariant},
};
use gpui::{
    App, FocusHandle, Focusable, KeyBinding, SharedString, Window, actions, div, prelude::*, px,
    uniform_list,
};
use strum::{EnumCount, IntoEnumIterator};

actions!(menu, [Escape, Enter, Next, Prev, Select]);

const CONTEXT: &str = "Menu";

pub struct Menu {
    variants: Vec<SharedString>,
    prev: Option<SharedString>,
    pub show: bool,
    count: usize,
    focused_item: usize,
    focus_handle: FocusHandle,
}

impl Menu {
    pub fn new(cx: &mut App) -> Self {
        cx.bind_keys([
            KeyBinding::new("escape", Escape, Some(CONTEXT)),
            KeyBinding::new("enter", Enter, Some(CONTEXT)),
            KeyBinding::new("up", Prev, Some(CONTEXT)),
            KeyBinding::new("k", Prev, Some(CONTEXT)),
            KeyBinding::new("down", Next, Some(CONTEXT)),
            KeyBinding::new("j", Next, Some(CONTEXT)),
            KeyBinding::new("enter", Select, Some(CONTEXT)),
        ]);

        let variants: Vec<SharedString> = ThemeVariant::iter()
            .map(|t| SharedString::from(t.to_string()))
            .collect();
        let current = cx.theme().variant.to_string().into();
        let focused_item = Menu::index_of(&variants, &current);

        Self {
            variants,
            prev: None,
            show: false,
            count: ThemeVariant::COUNT,
            focused_item,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn focus(&self, window: &mut Window) {
        self.focus_handle.focus(window)
    }

    pub fn is_focused(&self, window: &mut Window) -> bool {
        self.focus_handle.is_focused(window)
    }

    fn update(&mut self, val: SharedString, cx: &mut Context<Self>, toggle: bool) {
        self.focused_item = Menu::index_of(&self.variants, &val);

        // prevents fs access on tests
        #[cfg(not(test))]
        Theme::update(&val, cx);

        if toggle {
            self.toggle(cx);
        }
    }

    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        if self.show {
            self.show = false;
        } else {
            self.prev = Some(cx.theme().variant.to_string().into());
            self.show = true;
        }
    }

    pub fn escape(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.escape_key(&Escape, window, cx);
    }

    pub fn show(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.show_key(&Enter, window, cx);
    }

    fn escape_key(&mut self, _: &Escape, _window: &mut Window, cx: &mut Context<Self>) {
        self.show = false;
        if self.prev.is_some() {
            let current = self.prev.clone().unwrap_or("Dark".into());
            self.focused_item = Menu::index_of(&self.variants, &current);
            self.update(current, cx, false);
        }
        cx.notify();
    }

    fn show_key(&mut self, _: &Enter, _window: &mut Window, cx: &mut Context<Self>) {
        self.show = true;
        self.prev = Some(cx.theme().variant.to_string().into());
        cx.notify();
    }

    fn select(&mut self, _: &Select, _window: &mut Window, cx: &mut Context<Self>) {
        self.update(self.variants[self.focused_item].clone(), cx, true);
        cx.notify();
    }

    fn next(&mut self, _: &Next, _window: &mut Window, cx: &mut Context<Self>) {
        if self.focused_item < (self.count - 1) {
            self.focused_item += 1;
        } else {
            self.focused_item = 0;
        }
        self.update(self.variants[self.focused_item].clone(), cx, false);
        cx.notify();
    }

    fn prev(&mut self, _: &Prev, _window: &mut Window, cx: &mut Context<Self>) {
        if self.focused_item == 0 {
            self.focused_item = self.count - 1;
        } else {
            self.focused_item -= 1;
        }
        self.update(self.variants[self.focused_item].clone(), cx, false);
        cx.notify();
    }

    fn index_of(variants: &[SharedString], val: &SharedString) -> usize {
        variants.iter().position(|v| v == val).unwrap_or(0)
    }
}

impl Render for Menu {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .key_context(CONTEXT)
            .when(self.show, |this| {
                this.on_action(cx.listener(Self::escape_key))
                    .on_action(cx.listener(Self::select))
                    .on_action(cx.listener(Self::next))
                    .on_action(cx.listener(Self::prev))
            })
            .when(!self.show, |this| {
                this.on_action(cx.listener(Self::show_key))
            })
            .track_focus(&self.focus_handle)
            .justify_start()
            .items_end()
            .p_2()
            .child(button(
                "menu",
                Icon::new(IconVariant::Theme, IconSize::Medium),
                cx,
                cx.listener(move |this, _, _, cx| this.toggle(cx)),
            ))
            .when(self.show, |this| {
                this.child(
                    div()
                        .flex()
                        .flex_col()
                        .absolute()
                        .top_10()
                        .w_40()
                        .h(px(168.))
                        .bg(cx.theme().field)
                        .rounded_md()
                        .p_1()
                        .child(
                            uniform_list(
                                cx.entity(),
                                "themes_list",
                                self.count,
                                |this, range, _window, cx| {
                                    range
                                        .map(|ix| {
                                            let item = this.variants[ix].clone();
                                            div()
                                                .rounded_md()
                                                .px_1()
                                                .hover(|this| this.bg(cx.theme().background))
                                                .when(this.focused_item == ix, |this| {
                                                    this.bg(cx.theme().background)
                                                })
                                                .child(text_button(
                                                    format!("theme_item_{ix}").as_str(),
                                                    item.clone(),
                                                    cx.listener(move |this, _, _window, cx| {
                                                        this.update(item.clone(), cx, true);
                                                    }),
                                                ))
                                        })
                                        .collect()
                                },
                            )
                            .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                                cx.stop_propagation();
                                cx.notify();
                                this.escape(window, cx);
                            }))
                            .h_full(),
                        ),
                )
            })
    }
}

impl Focusable for Menu {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Entity, TestAppContext, VisualTestContext};

    const MAX_INDEX: usize = 4;

    #[gpui::test]
    fn test_menu_update(cx: &mut TestAppContext) {
        let (menu, cx) = setup_menu(cx);
        let mut result: ThemeVariant = ThemeVariant::Light;

        menu.update(cx, |menu, cx| {
            menu.update("Light".into(), cx, true);
            result = cx.theme().variant.clone();
        });

        assert_eq!(ThemeVariant::Light, result);
    }

    #[gpui::test]
    fn test_menu_select(cx: &mut TestAppContext) {
        let (menu, cx) = setup_menu(cx);
        let mut show = true;
        let mut result = 0;
        menu.update(cx, |menu, _cx| {
            menu.show = show;
            menu.focused_item = result;
        });

        cx.focus(&menu);
        cx.simulate_keystrokes("j enter");
        menu.update(cx, |menu, _cx| {
            show = menu.show;
            result = menu.focused_item;
        });

        assert_eq!(1, result);
        assert_eq!(false, show)
    }

    #[gpui::test]
    fn test_menu_next_at_limit(cx: &mut TestAppContext) {
        let (menu, cx) = setup_menu(cx);
        menu.update(cx, |menu, _cx| {
            menu.show = true;
            menu.focused_item = MAX_INDEX;
        });
        let mut result = 0;

        cx.focus(&menu);
        cx.simulate_keystrokes("j");
        menu.update(cx, |menu, _cx| result = menu.focused_item);

        assert_eq!(0, result)
    }

    #[gpui::test]
    fn test_menu_prev_at_limit(cx: &mut TestAppContext) {
        let (menu, cx) = setup_menu(cx);
        menu.update(cx, |menu, _cx| {
            menu.show = true;
            menu.focused_item = 0;
        });
        let mut result = 0;

        cx.focus(&menu);
        cx.simulate_keystrokes("k");
        menu.update(cx, |menu, _cx| result = menu.focused_item);

        assert_eq!(MAX_INDEX, result)
    }

    fn setup_menu(cx: &mut TestAppContext) -> (Entity<Menu>, &mut VisualTestContext) {
        Theme::test(cx);
        cx.add_window_view(|_window, cx| Menu::new(cx))
    }
}
