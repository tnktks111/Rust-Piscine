use bigger::*;
use std::collections::HashMap;

#[test]
fn test_bigger() {
    let h = HashMap::from([
        ("Daniel", 122),
        ("Ashley", 333),
        ("Katie", 334),
        ("Robert", 14),
    ]);

    assert_eq!(bigger(h), 334);
}

#[test]
fn test_bigger_negative() {
    let h = HashMap::from([("A", -10), ("B", 20), ("C", 5)]);

    assert_eq!(bigger(h), 20);
}

#[test]
fn test_bigger_single() {
    let h = HashMap::from([("A", 42)]);

    assert_eq!(bigger(h), 42);
}
