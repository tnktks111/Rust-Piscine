pub fn invert_sentence(string: &str) -> String {
    let mut rev_words = string.split_whitespace().rev();
    let mut result = String::with_capacity(string.len());
    let mut in_word = false;

    for ch in string.chars() {
        if ch.is_whitespace() {
            result.push(ch);
            in_word = false;
        } else if !in_word {
            result.push_str(rev_words.next().unwrap());
            in_word = true;
        }
    }
    result
}
