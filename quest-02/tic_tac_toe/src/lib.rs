pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if diagonals('O', table) || horizontal('O', table) || vertical('O', table) {
        String::from("player O won")
    } else if diagonals('X', table) || horizontal('X', table) || vertical('X', table) {
        String::from("player X won")
    } else {
        String::from("tie")
    }
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    (table[0][0] == player && table[1][1] == player && table[2][2] == player)
        || (table[2][0] == player && table[1][1] == player && table[0][2] == player)
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for r in 0..3 {
        if table[r][0] == player && table[r][1] == player && table[r][2] == player {
            return true;
        }
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for c in 0..3 {
        if table[0][c] == player && table[1][c] == player && table[2][c] == player {
            return true;
        }
    }
    false
}
