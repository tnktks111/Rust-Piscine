pub fn initials(names: Vec<&str>) -> Vec<String> {    
    fn name2initial(name: &str) -> String {
        let mut initial = String::new();

        for (idx, word) in name.split_whitespace().enumerate() {
            if idx != 0 {
                initial.push(' ');
            }
            initial.extend(word.chars().next().unwrap().to_uppercase());
            initial.push('.');
        }
        initial
    }

    let mut results: Vec<String> = Vec::with_capacity(names.len());
    for name in names {
        results.push(name2initial(name));
    }
    results
}