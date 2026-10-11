use strings::*;

#[test]
fn test_char_length() {
    assert_eq!(char_length("hello"), 5);
    assert_eq!(char_length("こんにちは"), 5);
    assert_eq!(char_length("❤"), 1);
    assert_eq!(char_length("形聲字"), 3);
    assert_eq!(char_length("😍"), 1);
    assert_eq!(char_length(""), 0);
}
