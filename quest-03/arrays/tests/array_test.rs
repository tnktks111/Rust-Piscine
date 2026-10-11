use arrays::*;

#[test]
fn test_sum() {
    assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
    assert_eq!(sum(&[10, 20, 30]), 60);
    assert_eq!(sum(&[-1, -2, 3]), 0);
    assert_eq!(sum(&[]), 0);
}

#[test]
fn test_thirtytwo_tens() {
    let a = thirtytwo_tens();

    assert_eq!(a.len(), 32);
    assert_eq!(a, [10; 32]);
    assert_eq!(sum(&a), 320);
}
