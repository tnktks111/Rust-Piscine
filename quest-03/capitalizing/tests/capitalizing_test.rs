use capitalizing::*;

#[test]
fn test_capitalize_first() {
    assert_eq!(capitalize_first("hello"), "Hello");
    assert_eq!(capitalize_first("joe is missing"), "Joe is missing");
    assert_eq!(capitalize_first("Hello"), "Hello");
    assert_eq!(capitalize_first(""), "");
    assert_eq!(capitalize_first("rust"), "Rust");
    assert_eq!(capitalize_first("école"), "École");
    assert_eq!(capitalize_first("ßeta"), "SSeta");
}

#[test]
fn test_title_case() {
    assert_eq!(title_case("jill is leaving A"), "Jill Is Leaving A");
    assert_eq!(title_case("hello world"), "Hello World");
    assert_eq!(title_case("HELLO WORLD"), "HELLO WORLD");
    assert_eq!(title_case("hello"), "Hello");
    assert_eq!(title_case(""), "");
    assert_eq!(title_case("école française"), "École Française");
}

#[test]
fn test_change_case() {
    assert_eq!(change_case("heLLo THere"), "HEllO thERE");
    assert_eq!(change_case("Hello World"), "hELLO wORLD");
    assert_eq!(change_case("Rust"), "rUST");
    assert_eq!(change_case("ABC123xyz"), "abc123XYZ");
    assert_eq!(change_case(""), "");
    assert_eq!(change_case("こんにちはAbC"), "こんにちはaBc");
    assert_eq!(change_case("ß"), "SS");
}
