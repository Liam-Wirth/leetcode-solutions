use crate::Solution;
//TODO: Revisit! This one was a tough one that I had to resort to using the editorial to solve, not
//confident in, or very happy with that solution as a result. Would love to go back over this
//problem again in the future

//DFS
//procedure DFS(G, v) is
//    label v as discovered
//    for all directed edges from v to w that are in G.adjacentEdges(v) do
//        if vertex w is not labeled as discovered then
//            recursively call DFS(G, w)

const DIRS: [i32; 5] = [0, 1, 0, -1, 0];
impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let mut max_gold = 0;
        let rows = grid.len();
        let cols = grid[0].len();

        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] != 0 {
                    //   println!("NEXT CALL!");
                    max_gold = max_gold.max(Self::max_gold_dfs(&mut grid.clone(), row, col));
                    //   println!("CURRENT_MAX {max_gold}");
                }
            }
        }

        max_gold
    }
    //Shitty name I think
    fn max_gold_dfs(grid: &mut Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
        if row < 0 || col < 0 || row >= grid.len() || col >= grid[0].len() || grid[row][col] == 0 {
            return 0;
        }

        let old = grid[row][col];
        let mut max_gold = 0;
        grid[row][col] = 0;

        for i in 0..4 {
            let next_row = DIRS[i] + row as i32;
            let next_col = DIRS[i + 1] + col as i32;

            // println!("Next {next_row}, Col: {next_col}");
            let mut next = Self::max_gold_dfs(grid, next_row as usize, next_col as usize);
            max_gold = max_gold.max(next);
        }
        grid[row][col] = old;
        max_gold + old
    }
}
