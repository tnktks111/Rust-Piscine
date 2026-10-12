pub fn is_permutation(s1: &str, s2: &str) -> bool {
    fn sort_str(s: &str) -> Vec<char> {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort();
        chars
    }

    sort_str(s1) == sort_str(s2)
}
