// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    types::Type,
    ui::{
        comp::{
            button::{button, text_button},
            icon::{Icon, IconSize, IconVariant},
        },
        util::theme::ActiveTheme,
        view::table::data_table::MAX_ITEMS,
    },
};
use gpui::{
    App, FocusHandle, Focusable, KeyBinding, ScrollStrategy, SharedString, UniformListScrollHandle,
    Window, actions, deferred, div, prelude::*, px, uniform_list,
};
use std::ops::Range;
use strum::{EnumCount, IntoEnumIterator};

actions!(dropdown, [Escape, Enter, Next, Prev, Select]);

const CONTEXT: &str = "Dropdown";

pub struct Dropdown {
    types: Vec<SharedString>,
    pub current: SharedString,
    prev: Option<SharedString>,
    pub show: bool,
    count: usize,
    pub id: usize,
    focused_item: usize,
    focus_handle: FocusHandle,
    scroll_handle: UniformListScrollHandle,
}

impl Dropdown {
    pub fn new(id: usize, cx: &mut Context<Self>) -> Self {
        cx.bind_keys([
            KeyBinding::new("escape", Escape, Some(CONTEXT)),
            KeyBinding::new("enter", Enter, Some(CONTEXT)),
            KeyBinding::new("up", Prev, Some(CONTEXT)),
            KeyBinding::new("k", Prev, Some(CONTEXT)),
            KeyBinding::new("down", Next, Some(CONTEXT)),
            KeyBinding::new("j", Next, Some(CONTEXT)),
            KeyBinding::new("enter", Select, Some(CONTEXT)),
        ]);

        let types: Vec<SharedString> = Type::iter()
            .map(|t| SharedString::from(t.to_string()))
            .collect();
        let current: SharedString = "Whiskey".into();
        let focused_item = Dropdown::index_of(&types, &current);

        Self {
            types,
            current,
            prev: None,
            show: false,
            count: Type::COUNT,
            id,
            focused_item,
            focus_handle: cx.focus_handle(),
            scroll_handle: UniformListScrollHandle::new(),
        }
    }

    pub fn focus(&self, window: &mut Window) {
        self.focus_handle.focus(window)
    }

    pub fn is_focused(&self, window: &mut Window) -> bool {
        self.focus_handle.is_focused(window)
    }

    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        cx.stop_propagation();
        if self.show {
            self.show = false;
        } else {
            self.prev = Some(self.current.clone());
            self.show = true;
        }
    }

    fn update(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
        val: SharedString,
        toggle: bool,
    ) {
        self.focused_item = Dropdown::index_of(&self.types, &val);
        self.current = val;
        if toggle {
            self.toggle(cx);
        }
        self.focus_handle.focus(window);
    }

    fn escape(&mut self, _: &Escape, _window: &mut Window, cx: &mut Context<Self>) {
        self.show = false;
        if self.prev.is_some() {
            let current = self.prev.clone().unwrap_or("Whiskey".into());
            self.focused_item = Dropdown::index_of(&self.types, &current);
            self.current = current;
        }
        self.scroll();
        cx.notify();
    }

    fn show(&mut self, _: &Enter, _window: &mut Window, cx: &mut Context<Self>) {
        self.show = true;
        self.prev = Some(self.current.clone());
        cx.notify();
    }

    // types is guaranteed to be non-empty, so default to 0th type to avoid panicking

    fn select(&mut self, _: &Select, window: &mut Window, cx: &mut Context<Self>) {
        self.update(
            window,
            cx,
            self.types
                .get(self.focused_item)
                .unwrap_or(&self.types[0])
                .clone(),
            true,
        );
        cx.notify();
    }

    fn next(&mut self, _: &Next, window: &mut Window, cx: &mut Context<Self>) {
        if self.focused_item < (self.count - 1) {
            self.focused_item += 1;
        } else {
            self.focused_item = 0;
        }
        self.scroll();
        self.update(
            window,
            cx,
            self.types
                .get(self.focused_item)
                .unwrap_or(&self.types[0])
                .clone(),
            false,
        );
        cx.notify();
    }

    fn prev(&mut self, _: &Prev, window: &mut Window, cx: &mut Context<Self>) {
        if self.focused_item == 0 {
            self.focused_item = self.count - 1;
        } else {
            self.focused_item -= 1;
        }
        self.scroll();
        self.update(
            window,
            cx,
            self.types
                .get(self.focused_item)
                .unwrap_or(&self.types[0])
                .clone(),
            false,
        );
        cx.notify();
    }

    fn scroll(&mut self) {
        self.scroll_handle
            .scroll_to_item(self.focused_item, ScrollStrategy::Top);
    }

    fn index_of(types: &[SharedString], val: &SharedString) -> usize {
        types.iter().position(|v| v == val).unwrap_or(0)
    }
}

impl Render for Dropdown {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        deferred(
            div()
                .flex()
                .flex_col()
                .key_context(CONTEXT)
                .when(self.show, |this| {
                    this.on_action(cx.listener(Self::escape))
                        .on_action(cx.listener(Self::select))
                        .on_action(cx.listener(Self::next))
                        .on_action(cx.listener(Self::prev))
                })
                .when(!self.show, |this| this.on_action(cx.listener(Self::show)))
                .track_focus(&self.focus_handle)
                .bg(cx.theme().field)
                .border_1()
                .border_color(cx.theme().field)
                .focus(|this| this.border_color(cx.theme().cursor))
                .px_2()
                .py_1()
                .rounded_md()
                .child(button(
                    &format!("dropdown_{}", self.id),
                    self.current.clone(),
                    Icon::new(IconVariant::Chevron, IconSize::Small),
                    cx.listener(move |this, _, _window, cx| {
                        this.toggle(cx);
                    }),
                ))
                .when(self.show, |this| {
                    this.child(
                        div()
                            .key_context(CONTEXT)
                            .flex()
                            .flex_col()
                            .absolute()
                            .top_9()
                            .right(px(0.))
                            .bg(cx.theme().field)
                            .rounded_md()
                            .p_1()
                            .w_full()
                            .h_48()
                            .child(
                                uniform_list(
                                    "ingreds_list",
                                    self.count,
                                    cx.processor(|this, range: Range<usize>, _window, cx| {
                                        range
                                            .map(|ix| {
                                                // 0th type is guranteed to exist, so this prevents
                                                // panicking if underlying uniform_list has a bug
                                                let item = this
                                                    .types
                                                    .get(ix)
                                                    .unwrap_or(&this.types[0])
                                                    .clone();
                                                div()
                                                    .rounded_md()
                                                    .px_1()
                                                    .hover(|this| this.bg(cx.theme().background))
                                                    .when(this.focused_item == ix, |this| {
                                                        this.bg(cx.theme().background)
                                                    })
                                                    .child(text_button(
                                                        &format!("dropdown_item_{ix}"),
                                                        item.clone(),
                                                        cx.listener(move |this, _, window, cx| {
                                                            this.update(
                                                                window,
                                                                cx,
                                                                item.clone(),
                                                                true,
                                                            );
                                                        }),
                                                    ))
                                            })
                                            .collect()
                                    }),
                                )
                                .track_scroll(self.scroll_handle.clone())
                                .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                                    cx.stop_propagation();
                                    this.escape(&Escape, window, cx);
                                }))
                                .h_full(),
                            ),
                    )
                }),
        )
        .with_priority(MAX_ITEMS - self.id)
    }
}

impl Focusable for Dropdown {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::ui::util::theme::Theme;

    use super::*;
    use gpui::{Entity, TestAppContext, VisualTestContext};

    const MAX_INDEX: usize = 17;

    #[gpui::test]
    fn test_dropdown_update(cx: &mut TestAppContext) {
        let (dropdown, cx) = setup_dropdown(cx);
        let mut result = String::new().into();

        dropdown.update_in(cx, |dropdown, window, cx| {
            dropdown.update(window, cx, "Vodka".into(), true);
            result = dropdown.current.clone();
        });

        assert_eq!(SharedString::from("Vodka"), result);
    }

    #[gpui::test]
    fn test_dropdown_select(cx: &mut TestAppContext) {
        let (dropdown, cx) = setup_dropdown(cx);
        dropdown.update(cx, |menu, _cx| menu.show = true);
        let mut result = String::new().into();

        cx.focus(&dropdown);
        cx.simulate_keystrokes("j enter");
        dropdown.update(cx, |dropdown, _cx| result = dropdown.current.clone());

        assert_eq!(SharedString::from("Vodka"), result);
    }

    #[gpui::test]
    fn test_dropdown_next_at_limit(cx: &mut TestAppContext) {
        let (dropdown, cx) = setup_dropdown(cx);
        dropdown.update(cx, |menu, _cx| {
            menu.show = true;
            menu.focused_item = MAX_INDEX;
        });
        let mut result = 0;

        cx.focus(&dropdown);
        cx.simulate_keystrokes("j");
        dropdown.update(cx, |dropdown, _cx| result = dropdown.focused_item);

        assert_eq!(0, result)
    }

    #[gpui::test]
    fn test_dropdown_prev_at_limit(cx: &mut TestAppContext) {
        let (dropdown, cx) = setup_dropdown(cx);
        dropdown.update(cx, |dropdown, _cx| {
            dropdown.show = true;
            dropdown.focused_item = 0;
        });
        let mut result = 0;

        cx.focus(&dropdown);
        cx.simulate_keystrokes("k");
        dropdown.update(cx, |dropdown, _cx| result = dropdown.focused_item);

        assert_eq!(MAX_INDEX, result)
    }

    fn setup_dropdown(cx: &mut TestAppContext) -> (Entity<Dropdown>, &mut VisualTestContext) {
        Theme::test(cx);
        cx.add_window_view(|_window, cx| Dropdown::new(0, cx))
    }
}
