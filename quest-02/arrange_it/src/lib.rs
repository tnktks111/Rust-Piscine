use std::cmp::Ordering;

fn compare(a: &&str, b: &&str) -> Ordering {
    let digit_a: u32 = a
        .bytes()
        .filter(|b| b.is_ascii_digit())
        .fold(0, |acc, b| acc * 10 + (b - b'0') as u32);
    let digit_b: u32 = b
        .bytes()
        .filter(|b| b.is_ascii_digit())
        .fold(0, |acc, b| acc * 10 + (b - b'0') as u32);

    digit_a.cmp(&digit_b)
}

pub fn arrange_phrase(phrase: &str) -> String {
    let mut result: String = String::with_capacity(phrase.len());

    let mut words: Vec<&str> = phrase.split_whitespace().collect::<Vec<&str>>();

    words.sort_by(compare);

    for (idx, word) in words.into_iter().enumerate() {
        if idx != 0 {
            result.push(' ');
        }
        result.extend(word.chars().filter(|c| !c.is_ascii_digit()));
    }

    result
}
