// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::types::{match_category, Category};

fn carbonated(percentage: f32) -> f32 {
    let raw = 1772.6 * percentage.powf(-0.996);
    return (raw * 100.0).round() / 100.0;
}

fn hard(percentage: f32) -> f32 {
    let raw = 3062.9 * percentage.powf(-1.161);
    return (raw * 100.0).round() / 100.0;
}

fn liqueur(percentage: f32) -> f32 {
    let raw = 235.94 * (percentage * -1.161).exp();
    return (raw * 100.0).round() / 100.0;
}

pub fn alc_weight(alc_type: &str, percentage: f32) -> (u32, f32) {
    let cat: Category = match_category(alc_type);
    match cat {
        Category::Carbonated => (1, carbonated(percentage)),
        Category::Liqueur => (2, liqueur(percentage)),
        Category::Hard => (3, hard(percentage)),
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
}
