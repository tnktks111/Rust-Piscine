use groceries::*;

#[test]
fn test_insert() {
    let mut groceries = vec![
        "yogurt".to_string(),
        "panettone".to_string(),
        "bread".to_string(),
        "cheese".to_string(),
    ];

    insert(&mut groceries, String::from("nuts"));

    assert_eq!(
        groceries,
        vec!["yogurt", "panettone", "bread", "cheese", "nuts"]
    );
}

#[test]
fn test_at_index() {
    let groceries = vec![
        "yogurt".to_string(),
        "panettone".to_string(),
        "bread".to_string(),
        "cheese".to_string(),
    ];

    assert_eq!(at_index(&groceries, 0), "yogurt");
    assert_eq!(at_index(&groceries, 1), "panettone");
    assert_eq!(at_index(&groceries, 3), "cheese");
}

#[test]
fn test_insert_empty() {
    let mut groceries = Vec::new();

    insert(&mut groceries, String::from("nuts"));

    assert_eq!(groceries, vec!["nuts"]);
}

#[test]
#[should_panic]
fn test_at_index_out_of_bounds() {
    let groceries = vec!["yogurt".to_string()];

    at_index(&groceries, 5);
}
