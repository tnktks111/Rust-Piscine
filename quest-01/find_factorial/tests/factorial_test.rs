use find_factorial::*;

#[test]
fn test_zero() {
    assert_eq!(factorial(0), 1);
}

#[test]
fn test_one() {
    assert_eq!(factorial(1), 1);
}

#[test]
fn test_five() {
    assert_eq!(factorial(5), 120);
}

#[test]
fn test_ten() {
    assert_eq!(factorial(10), 3628800);
}

#[test]
fn test_nineteen() {
    assert_eq!(factorial(19), 121645100408832000);
}
