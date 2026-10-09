use std::io::Write;
use std::process::{Command, Stdio};

const RIDDLE: &str = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";

fn run(input: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_looping"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();

    assert!(output.status.success());

    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn test_first_try() {
    let output = run("The letter e\n");

    let expected = format!(
        "{RIDDLE}\nNumber of trials: 1\n"
    );

    assert_eq!(output, expected);
}

#[test]
fn test_multiple_tries() {
    let output = run("wrong\nI don't know\nThe letter e\n");

    let expected = format!(
        "{RIDDLE}\n{RIDDLE}\n{RIDDLE}\nNumber of trials: 3\n"
    );

    assert_eq!(output, expected);
}