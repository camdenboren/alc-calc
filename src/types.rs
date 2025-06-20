// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use std::str::FromStr;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

#[derive(EnumString, Clone, EnumCount, EnumIter, Display)]
pub enum Type {
    Hard,
    Whiskey,
    Vodka,
    Gin,
    Rum,
    Everclear,
    GrainAlcohol,
    Flavored,
    Liqueur,
    Baileys,
    Schnapps,
    Kahlua,
    Carbonated,
    Fermented,
    Beer,
    Wine,
    MaltBeer,
    Seltzer,
}

#[derive(Debug, PartialEq)]
pub enum Category {
    Carbonated,
    Liqueur,
    Hard,
}

pub fn match_category(alc_type: &str) -> Category {
    let alc_type_e: Type = Type::from_str(alc_type).unwrap_or(Type::Whiskey);
    match alc_type_e {
        Type::Hard => Category::Hard,
        Type::Whiskey => Category::Hard,
        Type::Vodka => Category::Hard,
        Type::Gin => Category::Hard,
        Type::Rum => Category::Hard,
        Type::Everclear => Category::Hard,
        Type::GrainAlcohol => Category::Hard,
        Type::Flavored => Category::Liqueur,
        Type::Liqueur => Category::Liqueur,
        Type::Baileys => Category::Liqueur,
        Type::Schnapps => Category::Liqueur,
        Type::Kahlua => Category::Liqueur,
        Type::Carbonated => Category::Carbonated,
        Type::Fermented => Category::Carbonated,
        Type::Beer => Category::Carbonated,
        Type::Wine => Category::Carbonated,
        Type::MaltBeer => Category::Carbonated,
        Type::Seltzer => Category::Carbonated,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_ingred_weight() {
        let cat: Category = match_category("Hard");
        assert_eq!(cat, Category::Hard);
    }
}
