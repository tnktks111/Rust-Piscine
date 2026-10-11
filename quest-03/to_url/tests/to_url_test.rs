use to_url::*;

#[test]
fn test_to_url() {
    assert_eq!(to_url("Hello, world!"), "Hello,%20world!");
    assert_eq!(to_url("a b c"), "a%20b%20c");
    assert_eq!(to_url("hello"), "hello");
    assert_eq!(to_url(""), "");
    assert_eq!(to_url("  "), "%20%20");
    assert_eq!(to_url("hello\tworld"), "hello\tworld");
    assert_eq!(to_url("こんにちは 世界"), "こんにちは%20世界");
}
