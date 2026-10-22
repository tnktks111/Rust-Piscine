use json::object;
use std::num::ParseFloatError;

pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

#[derive(Debug)]
enum CalorieError {
    InvalidUnit,
    InvalidNumber(ParseFloatError),
}

fn parse_calories(s: &str) -> Result<f64, CalorieError> {
    let s = s.trim();

    let (num, factor) = if let Some(num) = s.strip_suffix("kJ") {
        (num, 1.0 / 4.184)
    } else if let Some(num) = s.strip_suffix("kcal") {
        (num, 1.0)
    } else {
        return Err(CalorieError::InvalidUnit);
    };

    let value: f64 = num.trim().parse().map_err(CalorieError::InvalidNumber)?;

    Ok(value * factor)
}

fn round2(n: f64) -> f64 {
    (n * 100.0).round() / 100.0
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut cals = 0.0;
    let mut carbs = 0.0;
    let mut proteins = 0.0;
    let mut fats = 0.0;

    for food in foods {
        cals += parse_calories(&food.calories.1).unwrap() * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
    }

    object! {
        cals: round2(cals),
        carbs: round2(carbs),
        proteins: round2(proteins),
        fats: round2(fats),
    }
}
