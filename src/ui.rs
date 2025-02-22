// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod input;
pub mod titlebar;
use crate::calc::alc_weight;
use crate::ui::input::TextInput;
use crate::ui::titlebar::titlebar;
use gpui::{
    div, opaque_grey, prelude::*, rgb, AppContext, FocusHandle, FocusableView, Keystroke,
    SharedString, View, ViewContext, WindowContext,
};
use std::env::consts::OS;

pub struct UI {
    text: SharedString,
    num: u32,
    text_input: View<TextInput>,
    pub recent_keystrokes: Vec<Keystroke>,
    focus_handle: FocusHandle,
}

impl FocusableView for UI {
    fn focus_handle(&self, _: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl UI {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        let (numm, _weight) = alc_weight("Liqueur", 40.0);
        let text_input = cx.new_view(|cx| TextInput {
            focus_handle: cx.focus_handle(),
            content: "".into(),
            placeholder: "Type here...".into(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            is_selecting: false,
        });
        cx.new_view(|cx| UI {
            text: "calc".into(),
            num: numm,
            text_input,
            recent_keystrokes: vec![],
            focus_handle: cx.focus_handle(),
        })
    }
}

impl Render for UI {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .font_family(".SystemUIFont")
            .bg(rgb(0x505050))
            .size_full()
            .shadow_lg()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(if OS == "linux" { titlebar() } else { div() })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .size_full()
                    .justify_center()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .size_full()
                            .justify_center()
                            .items_center()
                            .gap_3()
                            .max_w_1_2()
                            .max_h_1_4()
                            .bg(opaque_grey(0.2, 1.0))
                            .rounded_lg()
                            .child(format!("alc-{} {}", &self.text, &self.num))
                            .child(self.text_input.clone())
                            .children(self.recent_keystrokes.iter().rev().map(|ks| {
                                format!(
                                    "{:} {}",
                                    ks.unparse(),
                                    if let Some(key_char) = ks.key_char.as_ref() {
                                        format!("-> {:?}", key_char)
                                    } else {
                                        "".to_owned()
                                    }
                                )
                            })),
                    ),
            )
    }
}
