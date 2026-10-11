use name_initials::*;

#[test]
fn test_initials() {
    let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];

    assert_eq!(initials(names), vec!["H. P.", "S. E.", "J. L.", "B. O."]);
}

#[test]
fn test_multiple_names() {
    let names = vec!["John Ronald Reuel Tolkien", "Alan Turing", "Ada Lovelace"];

    assert_eq!(initials(names), vec!["J. R. R. T.", "A. T.", "A. L."]);
}

#[test]
fn test_edge_cases() {
    assert_eq!(initials(vec![""]), vec![""]);
    assert_eq!(initials(vec!["Alice"]), vec!["A."]);
    assert_eq!(initials(vec!["  John   Doe  "]), vec!["J. D."]);
    assert_eq!(initials(vec![]), Vec::<String>::new());
}
