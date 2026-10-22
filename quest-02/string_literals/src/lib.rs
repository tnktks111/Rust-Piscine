pub fn is_empty(v: &str) -> bool {
    v.len() == 0
}

pub fn is_ascii(v: &str) -> bool {
    for c in v.chars() {
        let c_val = c as u32;
        if c_val >= 128 {
            return false;
        }
    }
    return true;
}

pub fn contains(v: &str, pat: &str) -> bool {}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    (&v[..index], &v[index..])
}

pub fn find(v: &str, pat: char) -> usize {}
