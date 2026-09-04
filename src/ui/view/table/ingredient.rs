// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

// Adapted from: https://github.com/zed-industries/zed/blob/main/crates/gpui/examples/data_table.rs

use crate::ui::{
    ActiveCtrl,
    comp::{
        button::icon_button,
        dropdown::Dropdown,
        icon::{Icon, IconSize, IconVariant},
        input::text_input::TextInput,
        tooltip::Tooltip,
    },
    util::theme::ActiveTheme,
};
use gpui::{Entity, EventEmitter, Pixels, SharedString, Window, div, prelude::*, px};

/// The fields rendered for each ingredient cell in the data table (including the header) w/ their
/// associated `Tooltip` descriptions and width (in pixels)
///
/// Note that for everything other than `weight`, the `Tooltip` is only displayed in the table
/// header
pub const FIELDS: [(&str, &str, f32); 4] = [
    ("ingredient", "Type of ingredient (e.g., Whiskey)", 158.),
    (
        "percentage",
        "Percentage of alcohol in the ingredient",
        132.,
    ),
    (
        "parts",
        "Desired number of parts of this ingredient relative to others",
        132.,
    ),
    (
        "weight",
        "Calculated weight (in g) of this ingredient to pour in the drink",
        72.,
    ),
];

/// An `Ingredient` element containing children for the `ingred_type` (e.g., whiskey, liqueur,
/// etc.), `percentage_input`, `parts_input`, and `weight`. As this element collects the vast
/// majority of user input AND displays the resultant weight of each ingredient, it represents the
/// core of the UX
///
/// Additionally, the `id` is used directly for indexed ingredient removal, though it's also
/// propagated to `Dropdown` and `TextInput` to manage tab behavior (`Dropdown` also uses it for
/// deferred rendering)
pub struct Ingredient {
    pub ingred_type: Entity<Dropdown>,
    pub percentage_input: Entity<TextInput>,
    pub parts_input: Entity<TextInput>,
    pub weight: SharedString,
    pub id: usize,
}

impl Ingredient {
    /// Create an `Ingredient` with the given `id`, propagating it to both the child `Dropdown`
    /// and `TextInput` elements
    ///
    /// To properly handle indexed ingredient removal and tab behavior, the parent will need to do
    /// the following (omitted for brevity)
    /// - Subscribe to the `Remove` event when creating the ingredient and drop that particular
    ///   ingredient from the `Vec` when emitted (via mouse-interaction)
    /// - Call `show_cursor_and_hide_dd` for each ingredient when handling `Tab`/`TabPrev`
    /// - Keep the ingredient's `id` synced regardless of ingredient removal
    ///
    /// # Examples
    ///
    /// ```
    /// use alc_calc::ui::view::table::ingredient::Ingredient;
    /// use gpui::{Entity, Window, prelude::*};
    ///
    /// struct Table {
    ///     ingreds: Vec<Entity<Ingredient>>,
    /// }
    ///
    /// impl Table {
    ///     fn new(
    ///         window: &mut Window,
    ///         cx: &mut Context<Self>
    ///     ) -> Self {
    ///         let ingred = cx.new(|cx| Ingredient::new(
    ///             0,
    ///             window,
    ///             cx,
    ///         ));
    ///
    ///         Table {
    ///             ingreds: vec![ingred],
    ///         }
    ///     }
    /// }
    /// ```
    pub fn new(id: usize, window: &mut Window, cx: &mut Context<Self>) -> Self {
        // we have 3 items per ingred and tab_index 1 is num_drinks_input,
        // so multiply by 3 and offset by two (UI itself is tab_index 0)
        Self {
            ingred_type: cx.new(|cx| Dropdown::new(id, cx, id as isize * 3 + 2)),
            percentage_input: cx
                .new(|cx| TextInput::new(window, cx, "Type here...".into(), id as isize * 3 + 3)),
            parts_input: cx
                .new(|cx| TextInput::new(window, cx, "Type here...".into(), id as isize * 3 + 4)),
            weight: "0".into(),
            id,
        }
    }

    /// Render each of `Ingredient`'s child elements as a distinct `div` at the width specified
    /// in `FIELDS`
    ///
    /// Weight is truncated to prevent overflowing the `Table` when massive quantities of
    /// ingredients are entered (the result can instead be seen in the `Tooltip`)
    fn render_cell(&self, key: &str, width: Pixels) -> impl IntoElement {
        div().w(width).child(match key {
            "ingredient" => div().id("").child(self.ingred_type.clone()),
            "percentage" => div().id("").child(self.percentage_input.clone()),
            "parts" => div().id("").child(self.parts_input.clone()),
            "weight" => {
                let display_weight = self.weight.to_string() + "g";
                div()
                    .w(width) // needs to be set again to inform truncate() of width
                    .truncate()
                    .child(display_weight.clone())
                    .id(format!("{}-weight", self.id).into_element())
                    .tooltip(move |_window, cx| cx.new(|_cx| Tooltip::new(&display_weight)).into())
            }
            _ => div().id("").child("--"),
        })
    }

    /// Set `Ingredient`'s `weight` to the stringified input
    pub fn weight(&mut self, weight: f32) {
        self.weight = weight.to_string().into();
    }

    /// Hide the `ingred_type` dropdown and set the associated cursor to `visible` for percentage
    /// and parts
    ///
    /// Note that setting a cursor to `visible` does NOT actually cause that cursor to be
    /// visible on the next render-this can only happen when the associated `TextInput` is
    /// focused
    pub fn show_cursor_and_hide_dd(&mut self, cx: &mut Context<Self>) {
        self.ingred_type
            .update(cx, |ingred_type, cx| ingred_type.hide(cx));
        self.percentage_input
            .update(cx, |percentage, cx| percentage.show_cursor(cx));
        self.parts_input
            .update(cx, |parts, cx| parts.show_cursor(cx));
    }

    fn remove(&mut self, cx: &mut Context<Self>) {
        cx.emit(Remove {});
    }
}

impl Render for Ingredient {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let id = self.id;

        div()
            .flex()
            .flex_row()
            .border_b_1()
            .border_color(cx.theme().background)
            .py_1()
            .items_center()
            .justify_center()
            .gap_x_4()
            .child(
                div()
                    .flex()
                    .child(icon_button(
                        "remove",
                        Icon::new(cx, IconVariant::Minus, IconSize::Small),
                        cx.listener(move |this, _, _window, cx| this.remove(cx)),
                    ))
                    .id(format!("remove_button_{id}").into_element())
                    .tooltip(|_window, cx| {
                        cx.new(|cx| {
                            Tooltip::new("Remove this Ingredient")
                                .keybind(&format!("{}-r", cx.ctrl()))
                        })
                        .into()
                    }),
            )
            .children(FIELDS.map(|(key, _, width)| self.render_cell(key, px(width))))
    }
}

pub struct Remove {}

impl EventEmitter<Remove> for Ingredient {}

/// The data model for `Ingredient`, which enables passing all ingredient data primitives to the
/// calculation logic in a structured manner via a simple `.map().collect()`
#[derive(Clone)]
pub struct IngredientData {
    pub ingred_type: SharedString,
    pub percentage: f32,
    pub parts: f32,
    pub density: f32,
    pub volume: f32,
    pub weight: f32,
    pub intermediate_weight: f32,
}

impl Default for IngredientData {
    fn default() -> Self {
        Self {
            ingred_type: "".into(),
            percentage: 0.,
            parts: 0.,
            density: 0.,
            volume: 0.,
            weight: 0.,
            intermediate_weight: 0.,
        }
    }
}
