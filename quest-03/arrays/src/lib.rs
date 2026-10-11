pub fn sum(a: &[i32]) -> i32 {
    a.iter().fold(0, |sum, b| sum + b)
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}
