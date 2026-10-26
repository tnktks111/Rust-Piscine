use invert_sentence::*;

#[test]
fn test_invert_sentence() {
    assert_eq!(invert_sentence("Rust is Awesome"), "Awesome is Rust");

    assert_eq!(
        invert_sentence("    word1     word2  "),
        "    word2     word1  "
    );

    assert_eq!(invert_sentence("Hello, World!"), "World! Hello,");
}

#[test]
fn test_whitespace() {
    assert_eq!(invert_sentence("hello\tworld\nrust"), "rust\tworld\nhello");

    assert_eq!(invert_sentence("   "), "   ");
    assert_eq!(invert_sentence(""), "");
    assert_eq!(invert_sentence("hello"), "hello");
}

#[test]
fn test_unicode() {
    assert_eq!(
        invert_sentence("こんにちは 世界 Rust"),
        "Rust 世界 こんにちは"
    );
}
