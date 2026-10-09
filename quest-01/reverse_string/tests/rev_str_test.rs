use reverse_string::*;

#[test]
fn test_rev_str() {
    assert_eq!(rev_str("hello"), "olleh");
    assert_eq!(rev_str("Rust"), "tsuR");
    assert_eq!(rev_str(""), "");
    assert_eq!(rev_str("a"), "a");
    assert_eq!(rev_str("こんにちは"), "はちにんこ");
}
