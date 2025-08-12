use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let n = 9; // Sudoku board size

        for i in 0..n {
            let mut row_seen = HashSet::new();
            let mut col_seen = HashSet::new();

            for j in 0..n {
                // Check row
                if board[i][j] != '.' {
                    if row_seen.contains(&board[i][j]) {
                        return false;
                    }
                    row_seen.insert(board[i][j]);
                }

                // Check column
                if board[j][i] != '.' {
                    // Notice the swapped indices [j][i]
                    if col_seen.contains(&board[j][i]) {
                        return false;
                    }
                    col_seen.insert(board[j][i]);
                }
            }
        }

        // time to do some silly shit to do the 3x3 subgrids

        for i in (0..9).step_by(3) {
            for j in (0..9).step_by(3) {
                let mut seen = HashSet::new();

                for row in i..i + 3 {
                    for col in j..j + 3 {
                        let n = board[row][col]; // TYPE CASTING MOTHAFUCKA!!

                        if seen.contains(&n) && n != '.' {
                            return false;
                        }
                        seen.insert(n);
                    }
                }
            }
        }

        true
    }
}
