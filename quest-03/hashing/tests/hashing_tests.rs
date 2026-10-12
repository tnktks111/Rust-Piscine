use hashing::*;

#[test]
fn test_mean() {
    assert_eq!(mean(&[1, 2, 3, 4, 5]), 3.0);
    assert_eq!(mean(&[1, 2, 3, 4]), 2.5);
    assert_eq!(mean(&[-10, 10]), 0.0);
    assert_eq!(mean(&[i32::MAX, i32::MAX]), i32::MAX as f64);
}

#[test]
fn test_median() {
    assert_eq!(median(&[1, 3, 2]), 2);
    assert_eq!(median(&[1, 2, 3, 4]), 2);
    assert_eq!(median(&[4, 7, 5, 2, 5, 1, 3]), 4);
    assert_eq!(median(&[i32::MAX, i32::MAX]), i32::MAX);
    assert_eq!(median(&[i32::MIN, i32::MIN]), i32::MIN);
}

#[test]
fn test_mode() {
    assert_eq!(mode(&[4, 7, 5, 2, 5, 1, 3]), 5);
    assert_eq!(mode(&[1, 1, 2, 3]), 1);
    assert_eq!(mode(&[-1, -1, 0]), -1);
    assert_eq!(mode(&[42]), 42);
}
