use expected_variable::expected_variable;

#[test]
fn test_expected_variable() {
    assert_eq!(
        expected_variable("On_Point", "on_point"),
        Some("100%".to_string())
    );

    assert_eq!(
        expected_variable("soClose", "so_close"),
        Some("88%".to_string())
    );

    assert_eq!(
        expected_variable("something", "something_completely_different"),
        None
    );

    assert_eq!(
        expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch"),
        Some("67%".to_string())
    );

    // 50%ちょうどは None
    assert_eq!(expected_variable("ab", "abcd"), None);
}
