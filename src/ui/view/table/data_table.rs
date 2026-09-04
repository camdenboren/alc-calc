// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

// Adapted from: https://github.com/zed-industries/zed/blob/main/crates/gpui/examples/data_table.rs

use crate::{
    calc::calc_weights,
    ui::{
        ActiveCtrl,
        comp::{
            button::icon_button,
            dropdown::Dropdown,
            icon::{Icon, IconSize, IconVariant},
            input::text_input::TextInput,
            toast::{ToastVariant, toast},
            tooltip::Tooltip,
        },
        util::theme::ActiveTheme,
        view::table::ingredient::{FIELDS, Ingredient, IngredientData},
    },
};
use gpui::{
    App, Entity, EventEmitter, FocusHandle, Focusable, KeyBinding, SharedString, Window, actions,
    div, prelude::*, px,
};

actions!(table, [Add, Delete, Escape, RemoveKey]);

pub const CONTEXT: &str = "Table";
pub const MAX_ITEMS: usize = 10;

/// A `Table` element containing a `TextInput` for `num_drinks` and a vector of
/// `Ingredients` (the latter of which can be added or removed)
///
/// Importantly, `Table`
/// - Serves as the connector between the application's UI and calculation logic, as
///   it collects the `Ingredient` data (when ready), maps it to the `IngredientData`
///   model, passes these data to `calc_weights()`, and updates each `Ingredient`'s weight
///   with the result
/// - Internally caps the number of `Ingredient`s to `MAX_ITEMS` to prevent both
///   visual and performance-related issues with massive numbers of `Ingredient`s
/// - Manages indices for each `Ingredient` to enable removing any given ingredient (not
///   just the last one)
///
/// Finally, as the `Table` intersects the `UI` and most of the lower-level UI
/// components, there's significant complexity on the parent's side re. the management of
/// both parent and child events via subscriptions (i.e., `Tab`, `TabPrev`, `Toggle`, and
/// `Add`)
///
/// Particularly:
/// - When creating the `Table`, you'll need to show the `num_drinks` cursor via a
///   subscription anytime a `Tab`, `TabPrev`, or `Toggle` event is emitted by `UI`
/// - Anytime you add an `Ingredient` (which emits `Add`), you'll need to
///   refresh each `Ingredient`'s subscriptions to `Tab` and `TabPrev` so that the
///   dropdowns and text inputs respond correctly
pub struct Table {
    pub ingreds: Vec<Entity<Ingredient>>,
    pub num_drinks_input: Entity<TextInput>,
    num_drinks: f32,
    count: usize,
    init: bool,
    focus_handle: FocusHandle,
}

impl Table {
    /// Create a `Table` element and bind relevant keys
    ///
    /// To properly handle subscriptions to relevant events you'll need to
    /// - Call `show_num_drinks_cursor()` via subscriptions to `Tab`, `TabPrev`, and
    ///   `Toggle` events when creating the `Table`
    /// - Implement `on_add()` by calling `show_cursor_and_hide_dd()` to handle
    ///   `Ingredient`'s `Add` event
    ///
    /// # Examples
    ///
    /// ```
    /// use alc_calc::ui::view::table::data_table::Table;
    /// use gpui::{
    ///     Entity,
    ///     EventEmitter,
    ///     Subscription,
    ///     Window,
    ///     actions,
    ///     prelude::*
    /// };
    ///
    /// actions!(doc_ui, [Tab]);
    ///
    /// struct UI {
    ///     table: Entity<Table>,
    ///     subscriptions: Vec<Subscription>,
    /// }
    ///
    /// impl UI {
    ///     fn new(
    ///         window: &mut Window,
    ///         cx: &mut Context<Self>
    ///     ) -> Self {
    ///         let table = cx.new(|cx| {
    ///             Table::new(window, cx)
    ///         });
    ///         cx.subscribe(
    ///             &table,
    ///             |this: &mut UI, _table, _event, cx| {
    ///                 this.on_add(cx)
    ///             })
    ///             .detach();
    ///
    ///         UI {
    ///             table,
    ///             subscriptions: vec![
    ///                 /// `TabPrev` and `Toggle`
    ///                 /// omitted for brevity
    ///                 cx.subscribe_self(|
    ///                     this: &mut UI,
    ///                     Tab,
    ///                     cx
    ///                 | {
    ///                     this
    ///                         .table
    ///                         .update(
    ///                             cx,
    ///                             |table, cx| {
    ///                                 table
    ///                                     .show_num_drinks_cursor(cx)
    ///                             },
    ///                         )
    ///                 }),
    ///             ],
    ///         }
    ///     }
    ///
    ///     fn on_add(&mut self, cx: &mut Context<Self>) {
    ///         while self.subscriptions.len() > 3 {
    ///             self.subscriptions.pop();
    ///         }
    ///
    ///         self.subscriptions.append(&mut vec![
    ///             // `TabPrev` omitted for brevity
    ///             cx.subscribe_self(
    ///                 |this: &mut UI, Tab, cx|
    ///             {
    ///                 this
    ///                     .table
    ///                     .update(
    ///                         cx,
    ///                         |table, cx| {
    ///                             table.show_cursor_and_hide_dd(cx)
    ///                         });
    ///             }),
    ///         ]);
    ///     }
    /// }
    ///
    /// impl EventEmitter<Tab> for UI {}
    /// ```
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let ctrl = cx.ctrl();
        cx.bind_keys([
            KeyBinding::new(&format!("{ctrl}-i"), Add, Some(CONTEXT)),
            KeyBinding::new(&format!("{ctrl}-d"), Delete, Some(CONTEXT)),
            KeyBinding::new(&format!("{ctrl}-r"), RemoveKey, Some(CONTEXT)),
            KeyBinding::new("escape", Escape, Some(CONTEXT)),
        ]);

        Self {
            ingreds: vec![],
            num_drinks_input: cx.new(|cx| TextInput::new(window, cx, "Type here...".into(), 1)),
            num_drinks: 0.,
            count: 0,
            init: true,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Add an ingredient to the table (capping the total to `MAX_ITEMS`), subscribe the table
    /// to it's `Remove` event and emit `Add`, then decrement `count`
    fn add(&mut self, _: &Add, window: &mut Window, cx: &mut Context<Self>) {
        if self.count < MAX_ITEMS {
            let id = self.count;
            let ingred = cx.new(|cx| Ingredient::new(id, window, cx));

            // subscribe to Ingred's Remove event
            cx.subscribe(
                &ingred,
                |this: &mut Table, ingred: Entity<Ingredient>, _event, cx| {
                    this.remove(ingred.read(cx).id, cx)
                },
            )
            .detach();

            self.ingreds.push(ingred);
            self.count += 1;

            // instruct UI to subscribe new ingreds to its Tab, TabPrev events
            cx.emit(Add {});
        }
        cx.notify();
    }

    /// Delete the last item in the ingredient `Vec` (after ensuring we can) and decrement
    /// `count`, while also resetting the focus to the table if this ingredient is currently
    /// focused
    fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        if self.count > 0 {
            if self.parts(self.count - 1, cx).is_focused(window)
                || self.percentage(self.count - 1, cx).is_focused(window)
                || self.ingred_type(self.count - 1, cx).is_focused(window)
            {
                self.focus(&Escape, window, cx);
            }
            self.ingreds.pop();
            self.count -= 1;
        }
        cx.notify();
    }

    /// Remove the ingredient at index `ix` from the ingredient `Vec` (after ensuring we can)
    /// and decrement `count`, while updating each ingredient's associated `id` (as well as
    /// it's corresponding dropdown's `id`). Keeping the `id`s synced is essential for
    /// enabling:
    /// - Per-ingredient removal triggered by `Ingredient`'s `Remove` event
    /// - Tab index calculations
    /// - Dropdown rendering priority
    fn remove(&mut self, ix: usize, cx: &mut Context<Self>) {
        // prevents remove(ix) and ingreds[ix..] from panicking if ix is OOB
        if self.count > 0 && ix < self.count {
            self.ingreds.remove(ix);
            self.count -= 1;

            // update id's so that we can use them for indexed removal and dd deferral
            self.ingreds[ix..]
                .iter()
                .enumerate()
                .for_each(|(jx, ingred)| {
                    ingred.update(cx, |ingred, cx| {
                        ingred.id = jx + ix;
                        ingred.ingred_type.update(cx, |ingred_type, _cx| {
                            ingred_type.id = jx + ix;
                        });
                    })
                });
        }
    }

    /// Convenience wrapper around `remove()`, handling `Table` focus and accepting the
    /// `RemoveKey` event to enable keyboard-driven ingredient removal w/o having to pass the
    /// event externally
    fn remove_key(&mut self, _: &RemoveKey, window: &mut Window, cx: &mut Context<Self>) {
        for ix in 0..self.count {
            if self.ingred_type(ix, cx).is_focused(window)
                || self.parts(ix, cx).is_focused(window)
                || self.percentage(ix, cx).is_focused(window)
            {
                self.remove(ix, cx);
                self.focus(&Escape, window, cx);
                break;
            }
        }
        cx.notify();
    }

    /// Set the `num_drinks_input` cursor to `visible`
    ///
    /// Note that setting a cursor to `visible` does NOT actually cause that cursor to be
    /// visible on the next render-this can only happen when the associated `TextInput` is
    /// focused
    pub fn show_num_drinks_cursor(&mut self, cx: &mut Context<Self>) {
        self.num_drinks_input
            .update(cx, |num_drinks, cx| num_drinks.show_cursor(cx));
    }

    /// For each `Ingredient`, hide the `ingred_type` dropdown and set the associated cursor
    /// to `visible` for percentage and parts
    ///
    /// Note that setting a cursor to `visible` does NOT actually cause that cursor to be
    /// visible on the next render-this can only happen when the associated `TextInput` is
    /// focused
    pub fn show_cursor_and_hide_dd(&mut self, cx: &mut Context<Self>) {
        self.ingreds
            .iter()
            .for_each(|ingred| ingred.update(cx, |ingred, cx| ingred.show_cursor_and_hide_dd(cx)));
    }

    /// Determine whether the `Table`'s associated inputs are valid (and thus, ready for
    /// calculation)
    ///
    /// As both `calc` and `calc_weights` have robust error-handling, this largely just
    /// prevents garbage from being displayed in the weight field for ingredients. There's
    /// likely performance gains as well, however, since, otherwise, significant calculation
    /// logic could be executed on each render
    fn ready(&mut self, cx: &mut Context<Self>) -> bool {
        if self.ingreds.is_empty() {
            return false;
        }

        (0..self.count).all(|ix| {
            let percentage = self.parse_or_zero(&self.percentage(ix, cx).content);
            let parts = self.parse_or_zero(&self.parts(ix, cx).content);
            percentage > 0. && (self.count <= 1 || parts > 0.)
        })
    }

    /// Map the `Table`'s associated inputs to the data model (an `IngredientData` vector),
    /// before passing to `calc_weights()` and updating each `Ingredient`'s weight with the
    /// result
    ///
    /// In the unlikely event of receiving an error from `calc_weights()`, an `Error` toast
    /// will be displayed
    ///
    /// This function is effectively the point of intersection between the front-and-back-ends
    fn calc(&mut self, cx: &mut Context<Self>, num_drinks: f32) {
        let mut ingred_data: Vec<IngredientData> = (0..self.count)
            .map(|ix| IngredientData {
                ingred_type: self.ingred_type(ix, cx).current.clone(),
                percentage: self.parse_or_zero(&self.percentage(ix, cx).content),
                parts: self.parse_or_zero(&self.parts(ix, cx).content),
                ..Default::default()
            })
            .collect();

        let ingred_data = match calc_weights(&mut ingred_data, num_drinks) {
            Ok(ingred_data) => ingred_data,
            Err(e) => {
                toast(
                    cx,
                    ToastVariant::Error,
                    &format!("Failed to calculate ingredient weights due to error: {e}"),
                );
                return;
            }
        };

        self.ingreds.iter().enumerate().for_each(|(ix, ingred)| {
            ingred.update(cx, |ingred, _| {
                // default to 0th ingred as both vecs are nonempty due to ready check
                ingred.weight(ingred_data.get(ix).unwrap_or(&ingred_data[0]).weight);
            });
        })
    }

    fn num_drinks<'a>(&'a self, cx: &'a Context<Self>) -> &'a TextInput {
        self.num_drinks_input.read(cx)
    }

    // default to 0th ingred to prevent panicking due to unexpected missing ingreds
    // there will always be a 0th ingred as these methods are only called w/ either
    //   1. an explicit nonempty check, or
    //   2. within a (0..self.count) block, meaning an empty vec produces no calls

    fn ingred_type<'a>(&'a self, ix: usize, cx: &'a Context<Self>) -> &'a Dropdown {
        self.ingreds
            .get(ix)
            .unwrap_or(&self.ingreds[0])
            .read(cx)
            .ingred_type
            .read(cx)
    }

    fn parts<'a>(&'a self, ix: usize, cx: &'a Context<Self>) -> &'a TextInput {
        self.ingreds
            .get(ix)
            .unwrap_or(&self.ingreds[0])
            .read(cx)
            .parts_input
            .read(cx)
    }

    fn percentage<'a>(&'a self, ix: usize, cx: &'a Context<Self>) -> &'a TextInput {
        self.ingreds
            .get(ix)
            .unwrap_or(&self.ingreds[0])
            .read(cx)
            .percentage_input
            .read(cx)
    }

    fn parse_or_zero(&self, content: &SharedString) -> f32 {
        content.trim().parse().unwrap_or(0.)
    }

    fn focus(&mut self, _: &Escape, window: &mut Window, _cx: &mut Context<Self>) {
        self.focus_handle.focus(window);
    }
}

impl EventEmitter<Add> for Table {}

impl Render for Table {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // focus num_drinks_input and add ingred on launch
        if self.init {
            self.add(&Add, window, cx);
            self.num_drinks(cx).focus(window);
            self.init = false;
        }

        self.num_drinks = self.parse_or_zero(&self.num_drinks(cx).content);

        if self.ready(cx) {
            self.calc(cx, self.num_drinks);
        }

        div()
            .key_context(CONTEXT)
            .on_action(cx.listener(Self::focus))
            .on_action(cx.listener(Self::add))
            .on_action(cx.listener(Self::delete))
            .on_action(cx.listener(Self::remove_key))
            .track_focus(&self.focus_handle(cx))
            .flex()
            .flex_col()
            .bottom(px(55.))
            .gap_3()
            .items_center()
            // num_drinks input
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_shrink()
                    .p_4()
                    .justify_center()
                    .items_center()
                    .rounded_lg()
                    .bg(cx.theme().foreground)
                    .gap_1()
                    .child(
                        div()
                            .flex()
                            .bottom(px(0.5))
                            .pb_2()
                            .text_xs()
                            .border_b_1()
                            .justify_start()
                            .w(px(120. + 4. * 2.))
                            .border_color(cx.theme().background)
                            .child(div().child("Units".to_uppercase()).bottom(px(1.5)))
                            .id("units_label")
                            .tooltip(|_window, cx| {
                                cx.new(|_cx| {
                                    Tooltip::new(
                                        "Total desired number of units of alcohol in the drink",
                                    )
                                })
                                .into()
                            }),
                    )
                    .child(self.num_drinks_input.clone()),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .p_4()
                    .gap_2()
                    .bg(cx.theme().foreground)
                    .rounded_lg()
                    // header
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .ml_8()
                            .h_5()
                            .gap_x_4()
                            .overflow_hidden()
                            .text_color(cx.theme().text)
                            .bg(cx.theme().foreground)
                            .bottom(px(2.))
                            .text_xs()
                            .children(FIELDS.map(|(key, desc, width)| {
                                div()
                                    .whitespace_nowrap()
                                    .flex_shrink_0()
                                    .truncate()
                                    .w(px(width))
                                    .child(key.replace("_", " ").to_uppercase())
                                    .id(format!("{key}_label").into_element())
                                    .tooltip(|_window, cx| cx.new(|_cx| Tooltip::new(desc)).into())
                            })),
                    )
                    // ingreds
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .border_t_1()
                            .border_color(cx.theme().background)
                            .children(self.ingreds.clone()),
                    )
                    // + button
                    .child(
                        div().flex().pt_2().h_6().child(
                            div()
                                .child(icon_button(
                                    "add",
                                    Icon::new(cx, IconVariant::Plus, IconSize::Small),
                                    cx.listener(move |this, _, window, cx| {
                                        this.add(&Add, window, cx);
                                    }),
                                ))
                                .id("add_button")
                                .tooltip(|_window, cx| {
                                    cx.new(|cx| {
                                        Tooltip::new("Add an Ingredient")
                                            .keybind(&format!("{}-i", cx.ctrl()))
                                    })
                                    .into()
                                }),
                        ),
                    ),
            )
    }
}

impl Focusable for Table {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::ui::{ActiveCtrl, Ctrl, UI, tests::setup_ui, util::theme::Theme};

    use super::*;
    use gpui::{TestAppContext, VisualTestContext};

    #[gpui::test]
    fn test_table_remove(cx: &mut TestAppContext) {
        let (table, cx, _ctrl) = setup_table(cx);
        let mut num_ingreds = 0;

        table.update(cx, |table, cx| {
            table.remove(0, cx);
            num_ingreds = table.ingreds.len();
        });

        assert_eq!(0, num_ingreds);
    }

    #[gpui::test]
    fn test_table_delete_when_empty(cx: &mut TestAppContext) {
        let (table, cx, ctrl) = setup_table(cx);
        let mut num_ingreds = 0;

        cx.focus(&table);
        (0..2).for_each(|_| cx.simulate_keystrokes(&format!("{ctrl}-d")));
        table.update(cx, |table, _cx| num_ingreds = table.ingreds.len());

        assert_eq!(0, num_ingreds);
    }

    #[gpui::test]
    fn test_table_add_when_full(cx: &mut TestAppContext) {
        let (table, cx, ctrl) = setup_table(cx);
        let mut num_ingreds = 0;

        cx.focus(&table);
        (0..15).for_each(|_| cx.simulate_keystrokes(&format!("{ctrl}-i")));
        table.update(cx, |table, _cx| num_ingreds = table.ingreds.len());

        assert_eq!(MAX_ITEMS, num_ingreds);
    }

    #[gpui::test]
    fn test_table_remove_key_when_empty(cx: &mut TestAppContext) {
        let (ui, cx, ctrl) = setup_ui_and_table(cx);
        let mut num_ingreds = 0;

        cx.focus(&ui);
        cx.simulate_keystrokes(&format!("tab tab {ctrl}-r {ctrl}-r"));
        ui.update(cx, |ui, cx| {
            ui.table
                .update(cx, |table, _cx| num_ingreds = table.ingreds.len());
        });

        assert_eq!(0, num_ingreds);
    }

    #[gpui::test]
    fn test_table_calc_single_ingred(cx: &mut TestAppContext) {
        let (ui, cx, _ctrl) = setup_ui_and_table(cx);
        let mut weight = SharedString::from("");

        cx.focus(&ui);
        cx.simulate_keystrokes("tab 2 tab tab 4 0");
        ui.update(cx, |ui, cx| {
            ui.table.update(cx, |table, cx| {
                weight = table.ingreds[0].read(cx).weight.clone();
            });
        });

        assert_eq!(SharedString::from("84.6"), weight);
    }

    #[gpui::test]
    fn test_table_calc_multiple_ingreds(cx: &mut TestAppContext) {
        let (ui, cx, ctrl) = setup_ui_and_table(cx);
        let mut weight: Vec<SharedString> = vec!["".into(), "".into()];

        cx.focus(&ui);
        cx.simulate_keystrokes(&format!("tab {ctrl}-i 2 tab tab 4 0 tab 1 . 5"));
        cx.simulate_keystrokes("tab enter k k k k enter tab 1 6 . 5 tab 1");
        ui.update(cx, |ui, cx| {
            ui.table.update(cx, |table, cx| {
                weight[0] = table.ingreds[0].read(cx).weight.clone();
                weight[1] = table.ingreds[1].read(cx).weight.clone();
            });
        });

        assert_eq!(SharedString::from("66.3"), weight[0]);
        assert_eq!(SharedString::from("46.9"), weight[1]);
    }

    #[gpui::test]
    fn test_table_not_ready_when_empty(cx: &mut TestAppContext) {
        let (table, cx, ctrl) = setup_table(cx);
        let mut ready = true;

        cx.focus(&table);
        cx.simulate_keystrokes(&format!("{ctrl}-d"));
        table.update(cx, |table, cx| ready = table.ready(cx));

        assert_eq!(false, ready);
    }

    #[gpui::test]
    fn test_table_focus_next_ingred(cx: &mut TestAppContext) {
        let (ui, cx, ctrl) = setup_ui_and_table(cx);
        let mut ingred_focused = false;

        cx.focus(&ui);
        cx.simulate_keystrokes(&format!("tab tab {ctrl}-i"));
        (0..3).for_each(|_| cx.simulate_keystrokes(&format!("tab")));
        ui.update_in(cx, |ui, window, cx| {
            ui.table.update(cx, |table, cx| {
                ingred_focused = table.ingreds[1]
                    .read(cx)
                    .ingred_type
                    .read(cx)
                    .is_focused(window)
            });
        });

        assert_eq!(true, ingred_focused);
    }

    fn setup_ui_and_table(
        cx: &mut TestAppContext,
    ) -> (Entity<UI>, &mut VisualTestContext, SharedString) {
        let (ui, cx, ctrl) = setup_ui(cx);
        ui.update_in(cx, |ui, window, cx| {
            ui.table = cx.new(|cx| Table::new(window, cx))
        });

        (ui, cx, ctrl)
    }

    fn setup_table(
        cx: &mut TestAppContext,
    ) -> (Entity<Table>, &mut VisualTestContext, SharedString) {
        Theme::test(cx);
        let mut ctrl: SharedString = "".into();
        cx.update(|cx| {
            Ctrl::set(cx);
            ctrl = cx.ctrl();
        });

        let (table, cx) = cx.add_window_view(|window, cx| Table::new(window, cx));
        (table, cx, ctrl)
    }
}
