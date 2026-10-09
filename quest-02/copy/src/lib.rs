pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut v: Vec<String> = vec![];
    for num_str in a.split_whitespace() {
        let num = num_str.parse::<f64>().unwrap();
        v.push(num.exp().to_string());
    }
    (a, v.join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut v: Vec<f64> = vec![];
    for x in &b {
        v.push((*x as f64).abs().ln());
    }
    (b, v)
}
