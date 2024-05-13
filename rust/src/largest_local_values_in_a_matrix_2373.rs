use crate::Solution;
impl Solution {
    fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        println!("{n}");
        let mut out: Vec<Vec<i32>> = vec![vec![0; n - 2]; n - 2];
        for i in 1..(n - 1) {
            for j in 1..(n - 1) {
                let mut temp = 0;
                for k in (i - 1)..(i + 2) {
                    for l in (j - 1)..(j + 2) {
                        temp = temp.max(grid[k][l]);
                    }
                }
                out[i - 1][j - 1] = temp
            }
        }
        out
    }
}
