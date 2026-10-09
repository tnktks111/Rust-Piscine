pub fn first_subword(mut s: String) -> String {
    let mut first_subword_end_pos = s.len();

    for (idx, ch) in s.char_indices() {
        if ch == '_' {
            first_subword_end_pos = idx;
            break;
        }
        if idx != 0 && ch.is_uppercase() {
            first_subword_end_pos = idx;
            break;
        }
    }
    s.truncate(first_subword_end_pos);
    s
}
