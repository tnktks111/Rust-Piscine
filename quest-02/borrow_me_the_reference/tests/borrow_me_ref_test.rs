use borrow_me_the_reference::*;

#[test]
fn test_delete_and_backspace() {
    let cases = [
        ("helll-o", "hello"),
        ("he+lllo", "hello"),
        ("ab--", ""),
        ("a++bc", "a"),
        ("bpp--o+er+++sskroi-++lcw", "borrow"),
        ("", ""),
        ("hello", "hello"),
        ("あいう-え", "あいえ"),
    ];

    for (input, expected) in cases {
        let mut s = input.to_string();
        delete_and_backspace(&mut s);
        assert_eq!(s, expected, "input: {input}");
    }
}

#[test]
fn test_do_operations_basic() {
    let mut v = [
        "2+2".to_owned(),
        "3+2".to_owned(),
        "10-3".to_owned(),
        "5+5".to_owned(),
    ];

    do_operations(&mut v);

    assert_eq!(v, ["4", "5", "7", "10"]);
}

#[test]
fn test_do_operations_multiple() {
    let mut v = [
        "10-3-2".to_owned(),
        "1+2-1".to_owned(),
        "1-2+3".to_owned(),
        "100-20+5-10".to_owned(),
        "10+20-5+1".to_owned(),
    ];

    do_operations(&mut v);

    assert_eq!(v, ["5", "2", "2", "75", "26"]);
}

#[test]
fn test_do_operations_edge_cases() {
    let mut v = [
        "42".to_owned(),
        "0".to_owned(),
        "0-5".to_owned(),
        "-3+2".to_owned(),
        "-10+3-2".to_owned(),
        "".to_owned(),
    ];

    do_operations(&mut v);

    assert_eq!(v, ["42", "0", "-5", "-1", "-9", "0"]);
}

#[test]
fn test_do_operations_vec() {
    let mut v = vec!["100+200".to_owned(), "1000-999".to_owned()];

    do_operations(&mut v);

    assert_eq!(v, ["300", "1"]);
}
