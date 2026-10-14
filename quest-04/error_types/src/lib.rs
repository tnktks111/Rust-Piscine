use chrono::Utc;
// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        let date_text = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Self {
            form_values: (field_name, field_value),
            date: (date_text),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        fn is_valid_char_pass(s: &str) -> bool {
            let mut alpha_found = false;
            let mut num_found = false;
            let mut symbol_found = false;

            for c in s.chars() {
                if c.is_ascii_alphabetic() {
                    alpha_found = true;
                } else if c.is_ascii_digit() {
                    num_found = true;
                } else if c.is_ascii_punctuation() {
                    symbol_found = true;
                } else {
                    return false;
                }
            }
            alpha_found && num_found && symbol_found
        }

        let err_msg_user_name_empty: &'static str = "Username is empty";
        let err_msg_password_too_short: &'static str =
            "Password should be at least 8 characters long";
        let err_msg_invalid_char: &'static str =
            "Password should be a combination of ASCII numbers, letters and symbols";

        if self.name.is_empty() {
            return Err(FormError::new(
                "name",
                self.name.to_string(),
                err_msg_user_name_empty,
            ));
        }
        if self.password.chars().count() < 8 {
            return Err(FormError::new(
                "password",
                self.password.to_string(),
                err_msg_password_too_short,
            ));
        }
        if !is_valid_char_pass(&self.password) {
            return Err(FormError::new(
                "password",
                self.password.to_string(),
                err_msg_invalid_char,
            ));
        }
        Ok(())
    }
}
