#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    fn apply_cipher(s: &str) -> String {
        s.chars()
            .map(|c| {
                if c.is_ascii_uppercase() {
                    char::from(('Z' as u8) - ((c as u8) - ('A' as u8)))
                } else if c.is_ascii_lowercase() {
                    char::from(('z' as u8) - ((c as u8) - ('a' as u8)))
                } else {
                    c
                }
            })
            .collect()
    }

    let expected: String = apply_cipher(original);
    if expected == ciphered {
        Ok(())
    } else {
        Err(CipherError { expected })
    }
}
