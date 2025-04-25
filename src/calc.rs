// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    types::{match_category, Category},
    ui::table::IngredientData,
};

fn round_to_place(raw: f32, place: f32) -> f32 {
    let base: f32 = 10.;
    let scalar: f32 = base.powf(place);
    (raw * scalar).round() / scalar
}

fn calc_ingred_weight(alc_type: &str, percentage: f32) -> (u32, f32) {
    let cat: Category = match_category(alc_type);
    match cat {
        Category::Carbonated => (1, 1772.6 * percentage.powf(-0.996)),
        Category::Liqueur => (2, 235.94 * (percentage * -0.044).exp()),
        Category::Hard => (3, 3062.9 * percentage.powf(-1.161)),
    }
}

fn calc_volume(alc_type: u32, percentage: f32) -> f32 {
    match alc_type {
        1 => (40. / percentage) * 44.355,
        2 => (40. / percentage) * 44.355,
        3 => (5. / percentage) * 354.84,
        _ => 0.,
    }
}

fn calc_scalar(data: &mut [IngredientData], num_drinks: f32) -> f32 {
    num_drinks
        / data
            .iter()
            .fold(0., |sum, item| sum + item.intermediate_weight / item.weight)
}

pub fn calc_weights(data: &mut Vec<IngredientData>, num_drinks: f32) -> &mut Vec<IngredientData> {
    if data.len() > 1 {
        // factor in volume and number of parts when there's multiple ingreds
        let mut first = data[0].clone();
        data.iter_mut().enumerate().for_each(|(ix, item)| {
            let (alc_type, weight) = calc_ingred_weight(&item.alc_type, item.percentage);
            item.weight = weight;
            item.volume = calc_volume(alc_type, item.percentage);
            item.density = weight / item.volume;

            if ix == 0 {
                item.intermediate_weight = weight;
                first = item.clone();
            } else {
                item.intermediate_weight =
                    ((first.volume * item.parts) / first.parts) * item.density;
            }
        });

        let scalar = calc_scalar(data, num_drinks);
        data.iter_mut().for_each(|item| {
            item.weight = round_to_place(scalar * item.intermediate_weight, 1.0);
        });
    } else {
        // use calc_ingred_weight directly if there's only one ingredient
        let weight;
        (_, weight) = calc_ingred_weight(&data[0].alc_type, data[0].percentage);
        data[0].weight = round_to_place(weight * num_drinks, 1.0);
    }

    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_to_place() {
        let num = round_to_place(42.281, 1.0);
        assert_eq!(num, 42.3);
    }

    #[test]
    fn test_calc_ingred_weight() {
        let (num, weight) = calc_ingred_weight("Hard", 40.0);
        assert_eq!(num, 3);
        assert_eq!(weight, 42.280598);
    }

    #[test]
    fn test_calc_volume() {
        assert_eq!(calc_volume(1, 40.), 44.355);
    }

    #[test]
    fn test_calc_scalar() {
        let num_drinks = 2.;
        let mut data: Vec<IngredientData> = Vec::new();
        (0..2).for_each(|_| {
            data.push(IngredientData {
                weight: 50.,
                intermediate_weight: 50.,
                ..Default::default()
            })
        });

        let result = calc_scalar(&mut data, num_drinks);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_calc_weights_single_ingred() {
        let mut data: Vec<IngredientData> = Vec::new();
        data.push(IngredientData {
            alc_type: "Kahlua".into(),
            percentage: 20.,
            ..Default::default()
        });

        let result = calc_weights(&mut data, 1.)[0].weight;
        assert_eq!(result, 97.9);
    }

    #[test]
    fn test_calc_weights_multiple_ingreds() {
        let mut data: Vec<IngredientData> = Vec::new();
        data.push(IngredientData {
            alc_type: "Whiskey".into(),
            parts: 1.5,
            percentage: 40.,
            ..Default::default()
        });
        data.push(IngredientData {
            alc_type: "Wine".into(),
            parts: 1.,
            percentage: 16.5,
            ..Default::default()
        });

        let result = calc_weights(&mut data, 2.);
        assert_eq!(result[0].weight, 66.3);
        assert_eq!(result[1].weight, 46.9);
    }
}
