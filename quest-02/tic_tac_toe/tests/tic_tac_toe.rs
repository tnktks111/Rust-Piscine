use tic_tac_toe::*;

#[test]
fn test_tic_tac_toe() {
    let tie = [['O', 'X', 'O'], ['O', 'P', 'X'], ['X', '#', 'X']];

    let o_win = [['X', 'O', 'O'], ['X', 'O', 'O'], ['#', 'O', 'X']];

    let x_win = [['O', 'O', 'X'], ['O', 'X', 'O'], ['X', '#', 'X']];

    assert_eq!(tic_tac_toe(tie), "tie");
    assert_eq!(tic_tac_toe(o_win), "player O won");
    assert_eq!(tic_tac_toe(x_win), "player X won");
}

#[test]
fn test_directions() {
    let table = [['X', 'O', 'X'], ['O', 'X', 'O'], ['O', 'O', 'X']];

    assert!(diagonals('X', table));
    assert!(!horizontal('X', table));
    assert!(!vertical('X', table));
}
