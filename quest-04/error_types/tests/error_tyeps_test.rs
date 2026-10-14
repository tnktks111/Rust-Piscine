use error_types::*;

#[test]
fn test_validate() {
    let cases = [
        ("Alice", "abc123_!", true),
        ("Alice", "abcdefgh", false),
        ("Alice", "abcdef12", false),
        ("Alice", "abcdef_!", false),
        ("Alice", "abc123あ!", false),
        ("Alice", "a1!bcdef", true),
        ("Alice", "a1!", false),
        ("", "abc123_!", false),
    ];

    for (name, password, expected) in cases {
        let form = Form {
            name: name.to_string(),
            password: password.to_string(),
        };

        assert_eq!(
            form.validate().is_ok(),
            expected,
            "name: {name}, password: {password}"
        );
    }
}

#[test]
fn test_error_messages() {
    let form = Form {
        name: String::new(),
        password: "abc123_!".to_string(),
    };

    let err = form.validate().unwrap_err();

    assert_eq!(err.form_values, ("name", String::new()));
    assert_eq!(err.err, "Username is empty");
}

#[test]
fn test_password_errors() {
    let mut form = Form {
        name: "Alice".to_string(),
        password: "short".to_string(),
    };

    assert_eq!(
        form.validate().unwrap_err().err,
        "Password should be at least 8 characters long"
    );

    form.password = "abcdefgh".to_string();

    assert_eq!(
        form.validate().unwrap_err().err,
        "Password should be a combination of ASCII numbers, letters and symbols"
    );
}
