// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::types::{match_category, Category};
use crate::ui::ingredient::IngredientData;

fn round_to_place(raw: f32, place: f32) -> f32 {
    let base: f32 = 10.0;
    let scaler: f32 = base.powf(place);
    return (raw * scaler).round() / scaler;
}

fn calc_ingred_weight(alc_type: &str, percentage: f32) -> (u32, f32) {
    let cat: Category = match_category(alc_type);
    match cat {
        Category::Carbonated => (1, 1772.6 * percentage.powf(-0.996)),
        Category::Liqueur => (2, 235.94 * (percentage * -1.161).exp()),
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

fn calc_scalar(data: &mut Vec<IngredientData>, num_drinks: i32) -> f32 {
    let mut temp = 0.;
    for ix in 0..data.len() {
        temp += data[ix].intermediate_weight / data[ix].weight;
    }
    num_drinks as f32 / temp
}

pub fn calc_weights(data: &mut Vec<IngredientData>, num_drinks: i32) -> &mut Vec<IngredientData> {
    if data.len() > 1 {
        // factor in volume and number of parts when there's multiple ingreds
        for ix in 0..data.len() {
            let alc_type;
            let weight;
            (alc_type, weight) = calc_ingred_weight(&data[ix].alc_type, data[ix].percentage);
            data[ix].weight = weight;
            data[ix].volume = calc_volume(alc_type, data[ix].percentage);
            data[ix].density = weight / data[ix].volume;

            // use the first ingredient as the relative value for parts
            if ix == 0 {
                data[ix].intermediate_weight = weight;
            } else {
                data[ix].intermediate_weight =
                    ((data[0].volume * data[ix].parts) / data[0].parts) * data[ix].density;
            }
        }

        let scalar = calc_scalar(data, num_drinks);
        for ix in 0..data.len() {
            data[ix].weight = round_to_place(scalar * data[ix].intermediate_weight, 2.0);
        }
    } else {
        // use alc_weight directly if there's only one ingredient
        let weight;
        (_, weight) = calc_ingred_weight(&data[0].alc_type, data[0].percentage);
        data[0].weight = round_to_place(weight * num_drinks as f32, 2.0);
    }

    data
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::SharedString;

    #[test]
    fn test_round_to_place() {
        let num = round_to_place(42.281, 2.0);
        assert_eq!(num, 42.28);
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
        let num_drinks = 2;
        let mut data: Vec<IngredientData> = Vec::new();
        data.push(IngredientData::new());
        data.push(IngredientData::new());
        data[0].weight = 50.;
        data[1].weight = 50.;
        data[0].intermediate_weight = 50.;
        data[1].intermediate_weight = 50.;

        let result = calc_scalar(&mut data, num_drinks);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_calc_weights_single_ingred() {
        let mut data: Vec<IngredientData> = Vec::new();
        data.push(IngredientData::new());
        data[0].alc_type = SharedString::from("Whiskey");
        data[0].percentage = 40.;

        let result = calc_weights(&mut data, 1)[0].weight;
        assert_eq!(result, 42.28);
    }

    #[test]
    fn test_calc_weights_multiple_ingreds() {
        let mut data: Vec<IngredientData> = Vec::new();
        data.push(IngredientData::new());
        data.push(IngredientData::new());
        data[0].alc_type = SharedString::from("Whiskey");
        data[1].alc_type = SharedString::from("Whiskey");
        data[0].percentage = 40.;
        data[1].percentage = 40.;
        data[0].parts = 1.;
        data[1].parts = 1.;

        let result = calc_weights(&mut data, 2);
        assert_eq!(result[0].weight, 42.28);
        assert_eq!(result[1].weight, 42.28);
    }
}
