use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let (_, &max_val) = h.iter().max_by_key(|entry| *entry.1).unwrap();
    assert!(max_val > 0);
    max_val
    // *h.values().max().unwrap()
}
