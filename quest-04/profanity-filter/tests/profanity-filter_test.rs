use profanity_filter::*;

#[test]
fn test_check_ms() {
    assert_eq!(check_ms("hello there"), Ok("hello there"));
    assert_eq!(check_ms(""), Err("ERROR: illegal"));
    assert_eq!(check_ms("you are stupid"), Err("ERROR: illegal"));
    assert_eq!(check_ms("stupid"), Err("ERROR: illegal"));
}

#[test]
fn test_valid_messages() {
    assert_eq!(check_ms("Hello, world!"), Ok("Hello, world!"));
    assert_eq!(check_ms("Rust is great"), Ok("Rust is great"));
}
