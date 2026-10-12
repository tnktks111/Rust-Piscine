use simple_hash::*;

#[test]
fn test_word_frequency_counter() {
    let words = ["hello", "world", "hello", "rust", "world", "hello"];

    let result = word_frequency_counter(&words);

    assert_eq!(result["hello"], 3);
    assert_eq!(result["world"], 2);
    assert_eq!(result["rust"], 1);
}

#[test]
fn test_nb_distinct_words() {
    let words = ["apple", "banana", "apple", "orange"];

    let frequencies = word_frequency_counter(&words);

    assert_eq!(nb_distinct_words(&frequencies), 3);
}

#[test]
fn test_empty() {
    let words: [&str; 0] = [];

    let frequencies = word_frequency_counter(&words);

    assert!(frequencies.is_empty());
    assert_eq!(nb_distinct_words(&frequencies), 0);
}
