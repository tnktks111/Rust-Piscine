use macro_calculator::*;

#[test]
fn test_calculate_macros() {
    // 公式サンプル
    let foods = [
        Food {
            name: "big mac".to_owned(),
            calories: ("2133.84kJ".to_owned(), "510kcal".to_owned()),
            proteins: 27.0,
            fats: 26.0,
            carbs: 41.0,
            nbr_of_portions: 2.0,
        },
        Food {
            name: "pizza margherita".to_owned(),
            calories: ("1500.59kJ".to_owned(), "358.65kcal".to_owned()),
            proteins: 13.89,
            fats: 11.21,
            carbs: 49.07,
            nbr_of_portions: 4.9,
        },
    ];

    let result = calculate_macros(&foods);

    assert_eq!(result["cals"].as_f64(), Some(2777.39));
    assert_eq!(result["carbs"].as_f64(), Some(322.44));
    assert_eq!(result["proteins"].as_f64(), Some(122.06));
    assert_eq!(result["fats"].as_f64(), Some(106.93));
}

#[test]
fn test_single_food() {
    let foods = [Food {
        name: "test".to_owned(),
        calories: ("418.4kJ".to_owned(), "100kcal".to_owned()),
        proteins: 10.0,
        fats: 5.0,
        carbs: 20.0,
        nbr_of_portions: 2.0,
    }];

    let result = calculate_macros(&foods);

    assert_eq!(result["cals"].as_f64(), Some(200.0));
    assert_eq!(result["carbs"].as_f64(), Some(40.0));
    assert_eq!(result["proteins"].as_f64(), Some(20.0));
    assert_eq!(result["fats"].as_f64(), Some(10.0));
}

#[test]
fn test_rounding() {
    let foods = [Food {
        name: "rounding".to_owned(),
        calories: ("0kJ".to_owned(), "12.294kcal".to_owned()),
        proteins: 1.234,
        fats: 2.346,
        carbs: 3.456,
        nbr_of_portions: 1.0,
    }];

    let result = calculate_macros(&foods);

    assert_eq!(result["cals"].as_f64(), Some(12.29));
    assert_eq!(result["carbs"].as_f64(), Some(3.46));
    assert_eq!(result["proteins"].as_f64(), Some(1.23));
    assert_eq!(result["fats"].as_f64(), Some(2.35));
}

#[test]
fn test_fractional_portions() {
    let foods = [Food {
        name: "half portion".to_owned(),
        calories: ("418.4kJ".to_owned(), "100kcal".to_owned()),
        proteins: 20.0,
        fats: 10.0,
        carbs: 40.0,
        nbr_of_portions: 0.5,
    }];

    let result = calculate_macros(&foods);

    assert_eq!(result["cals"].as_f64(), Some(50.0));
    assert_eq!(result["carbs"].as_f64(), Some(20.0));
    assert_eq!(result["proteins"].as_f64(), Some(10.0));
    assert_eq!(result["fats"].as_f64(), Some(5.0));
}

#[test]
fn test_empty() {
    let foods: [Food; 0] = [];

    let result = calculate_macros(&foods);

    assert_eq!(result["cals"].as_f64(), Some(0.0));
    assert_eq!(result["carbs"].as_f64(), Some(0.0));
    assert_eq!(result["proteins"].as_f64(), Some(0.0));
    assert_eq!(result["fats"].as_f64(), Some(0.0));
}
