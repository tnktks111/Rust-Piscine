use arrange_it::*;

#[test]
fn test_arrange_phrase_basic() {
    assert_eq!(arrange_phrase("is2 Thi1s T4est 3a"), "This is a Test");
}

#[test]
fn test_arrange_phrase_order() {
    assert_eq!(
        arrange_phrase("world2 Hello1 Rust4 from3"),
        "Hello world from Rust"
    );

    assert_eq!(
        arrange_phrase("third3 first1 second2"),
        "first second third"
    );
}

#[test]
fn test_arrange_phrase_single_word() {
    assert_eq!(arrange_phrase("Ru1st"), "Rust");
    assert_eq!(arrange_phrase("hello1"), "hello");
}

#[test]
fn test_arrange_phrase_empty() {
    assert_eq!(arrange_phrase(""), "");
}

#[test]
fn test_arrange_phrase_whitespace() {
    assert_eq!(
        arrange_phrase("  is2  \tThi1s   T4est\n3a  "),
        "This is a Test"
    );
}

#[test]
fn test_arrange_phrase_unicode() {
    assert_eq!(arrange_phrase("世2界 こ1ん"), "こん 世界");
}

#[test]
fn test_arrange_phrase_multiple_digits() {
    assert_eq!(
        arrange_phrase("ten10 six6 one1 eight8 three3 five5 seven7 four4 two2 nine9"),
        "one two three four five six seven eight nine ten"
    );
}
