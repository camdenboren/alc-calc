// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use gpui::SharedString;

/// Allows a `SharedString` to have spaces inserted between words via
/// `shared_string.insert_spaces()`
pub trait Spaceable {
    fn insert_spaces(&self) -> Self;
}

impl Spaceable for SharedString {
    /// Insert spaces between lowercase and uppercase letters–particularly useful for
    /// displaying enums converted to strings via `strum_macros::Display`.
    ///
    /// # Examples
    ///
    /// ```
    /// use alc_calc::ui::util::str::Spaceable;
    /// use gpui::SharedString;
    ///
    /// let str = SharedString::from("ConvertedEnum");
    /// let expected = SharedString::from("Converted Enum");
    /// assert_eq!(str.insert_spaces(), expected);
    /// ```
    fn insert_spaces(&self) -> SharedString {
        let str = self.to_string();
        let mut chars = str.chars().peekable();
        let mut spaced = "".to_string();

        while let Some(e) = chars.next() {
            spaced.push(e);
            if let Some(pe) = chars.peek()
                && e.is_ascii_lowercase()
                && pe.is_ascii_uppercase()
            {
                spaced.push(' ');
            }
        }

        spaced.into()
    }
}
