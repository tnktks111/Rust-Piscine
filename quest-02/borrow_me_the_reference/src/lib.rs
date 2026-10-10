pub fn delete_and_backspace(s: &mut String) {
    let mut result: String = String::new();
    let mut delete_cnt = 0;

    for c in s.chars() {
        if c == '+' {
            delete_cnt += 1;
        } else if delete_cnt > 0 {
            delete_cnt -= 1;
        } else if c == '-' {
            result.pop();
        } else {
            result.push(c);
        }
    }

    *s = result;
}

pub fn do_operations(v: &mut [String]) {
    fn do_single_operation(s: &str) -> i32 {
        if s.is_empty() {
            return 0;
        }

        for (idx, ch) in s.char_indices().rev() {
            if ch == '+' {
                let right = &s[idx + 1..];
                return do_single_operation(&s[..idx]) + right.parse::<i32>().unwrap();
            }
            if idx != 0 && ch == '-' {
                let right = &s[idx + 1..];
                return do_single_operation(&s[..idx]) - right.parse::<i32>().unwrap();
            }
        }
        s.parse::<i32>().unwrap()
    }

    for x in v.iter_mut() {
        *x = do_single_operation(x).to_string();
    }
}
