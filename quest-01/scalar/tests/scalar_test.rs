use scalar::*;

#[test]
fn test_sum() {
    assert_eq!(sum(234, 2), 236);
}

#[test]
#[should_panic(expected = "attempt to add with overflow")]
fn test_sum_overflow() {
    sum(1, 255);
}

#[test]
fn test_diff() {
    assert_eq!(diff(234, 2), 232);
}

#[test]
#[should_panic(expected = "attempt to subtract with overflow")]
fn test_diff_overflow() {
    diff(-32768, 32766);
}

#[test]
fn test_pro() {
    assert_eq!(pro(23, 2), 46);
}

#[test]
#[should_panic(expected = "attempt to multiply with overflow")]
fn test_pro_overflow() {
    pro(-128, 2);
}

#[test]
fn test_quo() {
    assert_eq!(quo(22.0, 2.0), 11.0);
    assert!((quo(-128.23, 2.0) - (-64.115)).abs() < 1e-5);
}

#[test]
fn test_rem() {
    assert_eq!(rem(22.0, 2.0), 0.0);
    assert!((rem(-128.23, 2.0) - (-0.22999573)).abs() < 1e-5);
}