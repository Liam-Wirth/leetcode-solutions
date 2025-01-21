use crate::{top_k_frequent_elements_347, Solution};

// TODO: Revisit, I don't understand the problem too well and needed to rely on solutions
impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        // First robot wants to minimize the number of points collected by the second robot
        // Second robot wants to maximize the number of points it collects
        
        // Calculate prefix sums for both rows
        let top = Solution::prefix_row(&grid[0]);
        let bottom = Solution::prefix_row(&grid[1]);
        
        let mut min_second_robot = i64::MAX;
        let n = grid[0].len();
        
        // For each possible transition point where robot1 can go down
        for i in 0..n {
            // Calculate what second robot can get:
            // Either remaining part of top row (after transition point)
            // Or part of bottom row (before transition point)
            let top_remainder = if i + 1 < n {
                top[n - 1] - top[i]
            } else {
                0
            };
            
            let bottom_remainder = if i > 0 {
                bottom[i - 1]
            } else {
                0
            };
            
            // Second robot will choose the maximum of these two paths
            let second_robot_score = std::cmp::max(top_remainder, bottom_remainder);
            
            // First robot wants to minimize second robot's maximum score
            min_second_robot = std::cmp::min(min_second_robot, second_robot_score as i64);
        }
        
        min_second_robot
    }

    fn prefix_row(row: &Vec<i32>) -> Vec<i64> {
        let mut prefix_sum = vec![0; row.len()];
        prefix_sum[0] = row[0] as i64;
        for i in 1..row.len() {
            prefix_sum[i] = prefix_sum[i - 1] + row[i] as i64;
        }
        prefix_sum
    }
}
