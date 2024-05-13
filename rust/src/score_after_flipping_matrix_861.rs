use crate::Solution;
impl Solution {
    pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
        // Flip rows where the first element is 0
        for row in &mut grid {
            if row[0] == 0 {
                for bit in row.iter_mut() {
                    *bit = 1 - *bit;
                }
            }
        }

        // Flip columns where the count of 0s is greater than the count of 1s
        // Skipping first column as we already checked it above
        let cols = grid[0].len();
        for col in 1..cols {
            let mut num_zeros = 0;
            for row in &grid {
                if row[col] == 0 {
                    num_zeros += 1
                }
                //if number of zeros is larger than half of the cols height
                // then the amount of 0s will be larger than the amount of 1s
                if num_zeros > grid.len() / 2 {
                    for row in &mut grid {
                        row[col] = 1 - row[col];
                    }
                    break;
                }
            }
        }

        // Calculate the decimal value of each row and accumulate
        let score: i32 = grid
            .iter()
            .map(|row| row.iter().fold(0, |acc, &bit| acc * 2 + bit))
            .sum();

        score
    }
}
