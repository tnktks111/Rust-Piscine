use ownership::*;

#[test]
fn test_first_subword() {
    assert_eq!(first_subword("helloWorld".to_owned()), "hello");
    assert_eq!(first_subword("snake_case".to_owned()), "snake");
    assert_eq!(first_subword("CamelCase".to_owned()), "Camel");
    assert_eq!(first_subword("just".to_owned()), "just");
}

#[test]
fn test_edge_cases() {
    assert_eq!(first_subword("".to_owned()), "");
    assert_eq!(first_subword("Hello".to_owned()), "Hello");
    assert_eq!(first_subword("_hello".to_owned()), "");
    assert_eq!(first_subword("hello_world_test".to_owned()), "hello");
    assert_eq!(first_subword("helloWORLD".to_owned()), "hello");
}

#[test]
fn test_unicode() {
    assert_eq!(first_subword("こんにちはWorld".to_owned()), "こんにちは");
}
