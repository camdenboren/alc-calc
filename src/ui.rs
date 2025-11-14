// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

mod comp;
pub mod util;
pub mod view;

#[cfg(target_os = "macos")]
use crate::ui::util::app_menu::{app_dock_menu, app_menu};
use crate::ui::{
    comp::{
        input::text_input::{Copy, Cut, Paste, SelectAll},
        toast::Toast,
    },
    util::{
        theme::{ActiveTheme, Theme},
        window::{self, WindowBorder, window_border},
    },
    view::{menu::ThemeMenu, table::data_table::Table, titlebar::Titlebar},
};
use gpui::{
    App, ClipboardItem, Entity, FocusHandle, Focusable, Global, KeyBinding, PromptLevel,
    SharedString, Window, actions, deferred, div, prelude::*,
};

actions!(
    ui,
    [
        About,
        Quit,
        Hide,
        NewWindow,
        CloseWindow,
        Minimize,
        Toggle,
        Tab
    ]
);

const CONTEXT: &str = "UI";

struct Ctrl {
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

impl ActiveCtrl for App {
    fn ctrl(&self) -> SharedString {
        Ctrl::global(self)
    }
}

#[derive(Clone)]
pub struct UI {
    menu: Entity<ThemeMenu>,
    table: Entity<Table>,
    titlebar: Entity<Titlebar>,
    focus_handle: FocusHandle,
}

impl UI {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Toast::set(cx);
        Ctrl::set(cx);
        let ctrl = cx.ctrl();
        cx.bind_keys([
            KeyBinding::new(&format!("{ctrl}-q"), Quit, Some(CONTEXT)),
            KeyBinding::new(&format!("{ctrl}-t"), Toggle, Some(CONTEXT)),
            KeyBinding::new(&format!("{ctrl}-n"), NewWindow, Some(CONTEXT)),
            KeyBinding::new(&format!("{ctrl}-w"), CloseWindow, Some(CONTEXT)),
            KeyBinding::new("tab", Tab, Some(CONTEXT)),
            #[cfg(target_os = "macos")]
            KeyBinding::new(&format!("{ctrl}-h"), Hide, Some(CONTEXT)),
            #[cfg(target_os = "macos")]
            KeyBinding::new(&format!("{ctrl}-x"), Cut, Some(CONTEXT)),
            #[cfg(target_os = "macos")]
            KeyBinding::new(&format!("{ctrl}-c"), Copy, Some(CONTEXT)),
            #[cfg(target_os = "macos")]
            KeyBinding::new(&format!("{ctrl}-v"), Paste, Some(CONTEXT)),
            #[cfg(target_os = "macos")]
            KeyBinding::new(&format!("{ctrl}-a"), SelectAll, Some(CONTEXT)),
        ]);

        #[cfg(target_os = "macos")]
        cx.set_menus(app_menu());

        #[cfg(target_os = "macos")]
        cx.set_dock_menu(app_dock_menu());

        // prevents fs access on tests
        #[cfg(not(test))]
        Theme::set(cx);

        UI {
            menu: cx.new(ThemeMenu::new),
            table: cx.new(|cx| Table::new(window, cx)),
            titlebar: cx.new(|_| Titlebar::default()),
            focus_handle: cx.focus_handle(),
        }
    }

    fn quit(&mut self, _: &Quit, _window: &mut Window, cx: &mut Context<Self>) {
        cx.quit();
    }

    fn close(&mut self, _: &CloseWindow, window: &mut Window, _cx: &mut Context<Self>) {
        window.remove_window();
    }

    fn create(&mut self, _: &NewWindow, _window: &mut Window, cx: &mut Context<Self>) {
        window::new_window(cx);
    }

    fn hide(&mut self, _: &Hide, _window: &mut Window, cx: &mut Context<Self>) {
        cx.hide();
    }

    fn minimize(&mut self, _: &Minimize, window: &mut Window, _cx: &mut Context<Self>) {
        window.minimize_window();
    }

    fn about(&mut self, _: &About, window: &mut Window, cx: &mut Context<Self>) {
        let message = "alc-calc";
        let detail = "v0.0.1";
        let prompt = window.prompt(
            PromptLevel::Info,
            "alc-calc",
            Some("v0.0.1"),
            &["Copy", "Ok"],
            cx,
        );

        cx.spawn(async move |_, cx| {
            if let Ok(0) = prompt.await {
                let content = format!("{message}\n{detail}");
                cx.update(|cx| {
                    cx.write_to_clipboard(ClipboardItem::new_string(content));
                })
                .ok();
            }
        })
        .detach();
    }

    // hacky stubs to inform MenuItems that they have an .on_action()
    // since it doesn't seem to be propagating from TextInput
    fn cut(&mut self, _: &Cut, _window: &mut Window, _cx: &mut Context<Self>) {}
    fn copy(&mut self, _: &Copy, _window: &mut Window, _cx: &mut Context<Self>) {}
    fn paste(&mut self, _: &Paste, _window: &mut Window, _cx: &mut Context<Self>) {}
    fn select(&mut self, _: &SelectAll, _window: &mut Window, _cx: &mut Context<Self>) {}

    fn toggle(&mut self, _: &Toggle, window: &mut Window, cx: &mut Context<Self>) {
        if self
            .table
            .read(cx)
            .focus_handle(cx)
            .contains_focused(window, cx)
            || self.focus_handle.is_focused(window)
        {
            self.menu.read(cx).focus(window);
            self.menu.update(cx, |menu, cx| menu.show(window, cx));
        } else if self.menu.read(cx).is_focused(window) {
            if self.menu.read(cx).show {
                self.menu.update(cx, |menu, cx| menu.escape(window, cx));
            }
            self.table.read(cx).num_drinks_input.read(cx).focus(window);
        }
    }

    fn focus_next(&mut self, _: &Tab, window: &mut Window, cx: &mut Context<Self>) {
        if self.focus_handle.is_focused(window) {
            self.table.read(cx).num_drinks_input.read(cx).focus(window);
        }
    }
}

impl Render for UI {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let decorations = window.window_decorations();

        window_border().child(
            div()
                .key_context(CONTEXT)
                .on_action(cx.listener(Self::toggle))
                .on_action(cx.listener(Self::focus_next))
                .on_action(cx.listener(Self::quit))
                .on_action(cx.listener(Self::close))
                .on_action(cx.listener(Self::create))
                .when(cfg!(target_os = "macos"), |this| {
                    this.on_action(cx.listener(Self::hide))
                        .on_action(cx.listener(Self::minimize))
                        .on_action(cx.listener(Self::about))
                        .on_action(cx.listener(Self::cut))
                        .on_action(cx.listener(Self::copy))
                        .on_action(cx.listener(Self::paste))
                        .on_action(cx.listener(Self::select))
                })
                .font_family(".SystemUIFont")
                .bg(cx.theme().background)
                .size_full()
                .text_xl()
                .text_color(cx.theme().text)
                .map(|this| WindowBorder::rounding(this, decorations))
                .track_focus(&self.focus_handle(cx))
                .when(cfg!(not(target_os = "windows")), |this| {
                    this.child(deferred(self.titlebar.clone()).with_priority(999))
                })
                .child(deferred(self.menu.clone()).with_priority(998))
                .child(
                    div()
                        .flex()
                        .justify_center()
                        .child(deferred(Toast::global(cx)).with_priority(997)),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .size_full()
                        .justify_center()
                        .items_center()
                        .child(self.table.clone()),
                ),
        )
    }
}

impl Focusable for UI {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, VisualTestContext};

    #[gpui::test]
    fn test_ui_toggle_menu(cx: &mut TestAppContext) {
        let (ui, cx, ctrl) = setup_ui(cx);
        let mut show_menu = false;

        cx.focus(&ui);
        cx.simulate_keystrokes(&format!("{ctrl}-t"));
        ui.update(cx, |ui, cx| show_menu = ui.menu.read(cx).show);

        assert_eq!(true, show_menu)
    }

    #[gpui::test]
    fn test_ui_toggle_table(cx: &mut TestAppContext) {
        let (ui, cx, ctrl) = setup_ui(cx);
        let mut show_menu = false;

        cx.focus(&ui);
        (0..2).for_each(|_| cx.simulate_keystrokes(&format!("{ctrl}-t")));
        ui.update(cx, |ui, cx| show_menu = ui.menu.read(cx).show);

        assert_eq!(false, show_menu)
    }

    #[gpui::test]
    fn test_ui_focus(cx: &mut TestAppContext) {
        let (ui, cx, _ctrl) = setup_ui(cx);
        let mut table_focused = false;

        cx.focus(&ui);
        cx.simulate_keystrokes("tab");
        ui.update_in(cx, |ui, window, cx| {
            table_focused = ui
                .table
                .read(cx)
                .num_drinks_input
                .read(cx)
                .is_focused(window)
        });

        assert_eq!(true, table_focused)
    }

    fn setup_ui(cx: &mut TestAppContext) -> (Entity<UI>, &mut VisualTestContext, SharedString) {
        Theme::test(cx);
        let mut ctrl: SharedString = "".into();
        cx.update(|cx| {
            Ctrl::set(cx);
            ctrl = cx.ctrl();
        });

        let (ui, cx) = cx.add_window_view(|window, cx| UI::new(window, cx));
        (ui, cx, ctrl)
    }
}
