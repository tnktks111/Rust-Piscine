use temperature_conv::*;

#[test]
fn test_temperature_conv() {
    let eps = 1e-10;

    assert!((fahrenheit_to_celsius(32.0) - 0.0).abs() < eps);
    assert!((fahrenheit_to_celsius(212.0) - 100.0).abs() < eps);
    assert!((fahrenheit_to_celsius(-40.0) - (-40.0)).abs() < eps);

    assert!((celsius_to_fahrenheit(0.0) - 32.0).abs() < eps);
    assert!((celsius_to_fahrenheit(100.0) - 212.0).abs() < eps);
    assert!((celsius_to_fahrenheit(-40.0) - (-40.0)).abs() < eps);
}
