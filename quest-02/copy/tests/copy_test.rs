use copy::*;

#[test]
fn test_nbr_function() {
    let (original, exp, ln) = nbr_function(2);

    assert_eq!(original, 2);
    assert!((exp - 2.0_f64.exp()).abs() < 1e-10);
    assert!((ln - 2.0_f64.ln()).abs() < 1e-10);

    // Zero
    let (original, exp, ln) = nbr_function(0);

    assert_eq!(original, 0);
    assert_eq!(exp, 1.0);
    assert_eq!(ln, f64::NEG_INFINITY);

    // Negative
    let (original, exp, ln) = nbr_function(-2);

    assert_eq!(original, -2);
    assert!((exp - (-2.0_f64).exp()).abs() < 1e-10);
    assert!((ln - 2.0_f64.ln()).abs() < 1e-10);
}

#[test]
fn test_str_function() {
    let input = "1 2 4 5 6".to_string();

    let (original, result) = str_function(input.clone());

    assert_eq!(original, input);

    let expected = [
        1.0_f64.exp(),
        2.0_f64.exp(),
        4.0_f64.exp(),
        5.0_f64.exp(),
        6.0_f64.exp(),
    ];

    let actual: Vec<f64> = result
        .split_whitespace()
        .map(|s| s.parse::<f64>().unwrap())
        .collect();

    assert_eq!(actual.len(), expected.len());

    for (a, e) in actual.iter().zip(expected.iter()) {
        assert!((a - e).abs() < 1e-10);
    }
}

#[test]
fn test_vec_function() {
    let input = vec![1, 2, 4, 5, -2];

    let (original, result) = vec_function(input.clone());

    assert_eq!(original, input);

    let expected = [0.0, 2.0_f64.ln(), 4.0_f64.ln(), 5.0_f64.ln(), 2.0_f64.ln()];

    assert_eq!(result.len(), expected.len());

    for (a, e) in result.iter().zip(expected.iter()) {
        assert!((a - e).abs() < 1e-10);
    }
}

#[test]
fn test_vec_function_edge_cases() {
    let (original, result) = vec_function(vec![]);

    assert!(original.is_empty());
    assert!(result.is_empty());

    let (_, result) = vec_function(vec![0, i32::MIN]);

    assert_eq!(result[0], f64::NEG_INFINITY);
    assert!((result[1] - (2147483648.0_f64).ln()).abs() < 1e-10);
}
