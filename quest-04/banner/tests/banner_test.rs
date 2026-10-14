use banner::*;
use std::collections::HashMap;

#[test]
fn test_opt_flag() {
    let flag = Flag::opt_flag("division", "divides the values");

    assert_eq!(flag.short_hand, "-d");
    assert_eq!(flag.long_hand, "--division");
    assert_eq!(flag.desc, "divides the values");
}

#[test]
fn test_div() {
    assert_eq!(div("10", "2"), Ok("5".to_string()));
    assert_eq!(div("1.0", "2.0"), Ok("0.5".to_string()));
    assert_eq!(div("-10", "2"), Ok("-5".to_string()));

    assert!(div("abc", "2").is_err());
    assert!(div("10", "xyz").is_err());
}

#[test]
fn test_rem() {
    assert_eq!(rem("10", "3"), Ok("1".to_string()));
    assert_eq!(rem("2.0", "2.0"), Ok("0".to_string()));
    assert_eq!(rem("-10", "3"), Ok("-1".to_string()));

    assert!(rem("abc", "3").is_err());
    assert!(rem("10", "xyz").is_err());
}

#[test]
fn test_add_flag() {
    let mut handler = FlagsHandler {
        flags: HashMap::new(),
    };

    handler.add_flag(Flag::opt_flag("division", "division operation"), div);

    assert_eq!(handler.flags.len(), 2);

    assert!(handler.flags.contains_key("-d"));
    assert!(handler.flags.contains_key("--division"));
}

#[test]
fn test_exec_func() {
    let mut handler = FlagsHandler {
        flags: HashMap::new(),
    };

    handler.add_flag(Flag::opt_flag("division", "division operation"), div);

    handler.add_flag(Flag::opt_flag("remainder", "remainder operation"), rem);

    // Short flags
    assert_eq!(handler.exec_func("-d", &["10", "2"]), Ok("5".to_string()));

    assert_eq!(handler.exec_func("-r", &["10", "3"]), Ok("1".to_string()));

    // Long flags
    assert_eq!(
        handler.exec_func("--division", &["1.0", "2.0"]),
        Ok("0.5".to_string())
    );

    assert_eq!(
        handler.exec_func("--remainder", &["2.0", "2.0"]),
        Ok("0".to_string())
    );

    // Parse errors
    assert_eq!(
        handler.exec_func("--division", &["abc", "2.0"]),
        Err("invalid float literal".to_string())
    );

    assert_eq!(
        handler.exec_func("--remainder", &["2.0", "xyz"]),
        Err("invalid float literal".to_string())
    );
}

#[test]
fn test_exec_func_extra_arguments() {
    let mut handler = FlagsHandler {
        flags: HashMap::new(),
    };

    handler.add_flag(Flag::opt_flag("division", "division operation"), div);

    // Only the first two arguments should be used.
    assert_eq!(
        handler.exec_func("-d", &["10", "2", "999"]),
        Ok("5".to_string())
    );
}
