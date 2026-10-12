pub fn capitalize_first(input: &str) -> String {
    let mut result = String::with_capacity(input.len());

    for (idx, ch) in input.char_indices() {
        if idx == 0 {
            result.extend(ch.to_uppercase());
        } else {
            result.push(ch);
        }
    }
    result
}

// pub fn capitalize_first(input: &str) -> String {
//     let mut chars = input.chars();

//     match chars.next() {
//         Some(first) => first.to_uppercase().chain(chars).collect(),
//         None => String::new()
//     }
// }

pub fn title_case(input: &str) -> String {
    let mut result: String = String::with_capacity(input.len());

    for (idx, word) in input.split_whitespace().enumerate() {
        if idx != 0 {
            result.push(' ');
        }
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            result.extend(first.to_uppercase().chain(chars));
        }
    }
    result
}

pub fn change_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len());

    for ch in input.chars() {
        if ch.is_uppercase() {
            result.extend(ch.to_lowercase());
        } else if ch.is_lowercase() {
            result.extend(ch.to_uppercase());
        } else {
            result.push(ch);
        }
    }
    result
}
