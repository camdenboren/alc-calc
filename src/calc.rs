// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::types::{match_category, Category};

fn round_to_place(raw: f32, place: f32) -> f32 {
    let base: f32 = 10.0;
    let scaler: f32 = base.powf(place);
    return (raw * scaler).round() / scaler;
}

pub fn alc_weight(alc_type: &str, percentage: f32) -> (u32, f32) {
    let cat: Category = match_category(alc_type);
    match cat {
        Category::Carbonated => (1, round_to_place(1772.6 * percentage.powf(-0.996), 2.0)),
        Category::Liqueur => (2, round_to_place(235.94 * (percentage * -1.161).exp(), 2.0)),
        Category::Hard => (3, round_to_place(3062.9 * percentage.powf(-1.161), 2.0)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alc_weight() {
        let (num, weight) = alc_weight("Hard", 40.0);
        assert_eq!(num, 3);
        assert_eq!(weight, 42.28);
    }

    #[test]
    fn test_round_to_place() {
        let num = round_to_place(42.281, 2.0);
        assert_eq!(num, 42.28);
    }
}
