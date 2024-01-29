use crate::Solution;
// @lc code=start
impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        let mut i =0;
        nums.into_iter().map(|num| {
            i += num;
            i
        }).collect()
    }
}
// @lc code=end

