use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    assert!(!list.is_empty());
    let sum = list.iter().map(|&x| x as f64).sum::<f64>();
    sum / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    assert!(!list.is_empty());

    let mut sorted_list: Vec<i32> = list.to_vec();
    sorted_list.sort();

    let len = list.len();
    if len % 2 == 0 {
        let a = sorted_list[len / 2 - 1] as i64;
        let b = sorted_list[len / 2] as i64;
        ((a + b) / 2) as i32
    } else {
        sorted_list[len / 2]
    }
}

pub fn mode(list: &[i32]) -> i32 {
    assert!(!list.is_empty());
    let mut counts: HashMap<i32, u32> = HashMap::new();

    list.iter().for_each(|x| {
        *counts.entry(*x).or_insert(0) += 1;
    });

    let (&max_key, _) = counts.iter().max_by_key(|entry| *entry.1).unwrap();
    max_key
}
