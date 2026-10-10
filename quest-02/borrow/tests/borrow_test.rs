use borrow::str_len;

#[test]
fn test_borrow() {
    let s = "hello";
    let s1 = "camelCase".to_string();
    let s2 = "olá!";

    assert_eq!(5, str_len(s));
    assert_eq!(9, str_len(&s1));
    assert_eq!(4, str_len(s2));
}
