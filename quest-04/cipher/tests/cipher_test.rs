use cipher::*;

#[test]
fn test_cipher_valid() {
    assert_eq!(cipher("1Hello 2world!", "1Svool 2dliow!"), Ok(()));

    assert_eq!(cipher("abc", "zyx"), Ok(()));
    assert_eq!(cipher("ABC", "ZYX"), Ok(()));
    assert_eq!(cipher("", ""), Ok(()));
}

#[test]
fn test_cipher_invalid() {
    assert_eq!(
        cipher("1Hello 2world!", "svool"),
        Err(CipherError {
            expected: "1Svool 2dliow!".to_string()
        })
    );

    assert_eq!(
        cipher("abc", "abc"),
        Err(CipherError {
            expected: "zyx".to_string()
        })
    );
}

#[test]
fn test_cipher_special_chars() {
    assert_eq!(cipher("Hello, Rust! 123", "Svool, Ifhg! 123"), Ok(()));
}
