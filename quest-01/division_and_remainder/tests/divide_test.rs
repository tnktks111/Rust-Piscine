use division_and_remainder::*;

#[test]
fn test_divide() {
    assert_eq!(divide(9, 4), (2, 1));
    assert_eq!(divide(10, 3), (3, 1));
    assert_eq!(divide(8, 2), (4, 0));
    assert_eq!(divide(0, 5), (0, 0));
}

#[test]
fn test_divide_negative() {
    assert_eq!(divide(-9, 4), (-2, -1));
    assert_eq!(divide(9, -4), (-2, 1));
    assert_eq!(divide(-9, -4), (2, -1));
}

#[test]
#[should_panic(expected = "attempt to divide by zero")]
fn test_divide_by_zero() {
    divide(10, 0);
}

#[test]
fn test_divide_boundary() {
    assert_eq!(divide(i32::MAX, 1), (i32::MAX, 0));
    assert_eq!(divide(i32::MIN, 1), (i32::MIN, 0));
}
