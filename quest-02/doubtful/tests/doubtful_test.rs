use doubtful::*;

#[test]
fn main() {
    let mut s = "Hello".to_owned();
    assert_eq!("Hello", s);
    doubtful(&mut s);
    assert_eq!("Hello?", s);
}
