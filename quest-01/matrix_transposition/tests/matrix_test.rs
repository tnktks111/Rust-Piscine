use matrix_transposition::*;

#[test]
fn test_matrix() {
    let matrix = Matrix((1, 3), (4, 5));

    assert_eq!(transpose(matrix), Matrix((1, 4), (3, 5)))
}
