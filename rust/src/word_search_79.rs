use crate::Solution;
impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<char> = word.chars().collect();

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Solution::word_search_dfs(&mut board, &word, i, j) {
                    return true;
                }
            }
        }

        false
    }

    /*
    technically popping values off of the word vector values we've seen????
    word[1..]
    base
    case word.len() == 1 {
        return board[x][y] == word[0]
    }


    */

    fn word_search_dfs(board: &mut Vec<Vec<char>>, word: &[char], x: usize, y: usize) -> bool {
        if word.len() == 1 {
            return board[x][y] == word[0];
        }
        //some other basecase me thinks?
        if board[x][y] != word[0] {
            return false;
        }
        let neighborhood = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        board[x][y] = '~';

        for dir in neighborhood {
            let x = x as i32 + dir.0;
            let y = y as i32 + dir.1;
            if x < 0 || y < 0 {
                continue;
            };
            let x = x as usize;
            let y = y as usize;

            if x >= board.len() || y >= board[0].len() {
                continue;
            }
            if board[x][y] == '~' {
                continue;
            }
            if Solution::word_search_dfs(board, &word[1..], x, y) {
                board[x][y] = word[0];
                return true;
            }
        }
        board[x][y] = word[0];
        false
    }
}
