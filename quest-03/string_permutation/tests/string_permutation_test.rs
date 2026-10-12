use string_permutation::*;

#[test]
fn test_is_permutation() {
    assert!(is_permutation("thought", "thougth"));
    assert!(is_permutation("abc", "bca"));
    assert!(is_permutation("aabbcc", "abcabc"));
    assert!(is_permutation("", ""));
    assert!(is_permutation("あいう", "ういあ"));

    assert!(!is_permutation("abc", "abd"));
    assert!(!is_permutation("aab", "abb"));
    assert!(!is_permutation("abc", "abcd"));
    assert!(!is_permutation("ABC", "abc"));
}
