// @lc code=start
use crate::Solution;
 impl Solution {
    pub fn build_array(mut nums: Vec<i32>) -> Vec<i32> {
        let l = nums.len() as i32;
        (0..l as usize).for_each(|i| nums[i] += (nums[nums[i] as usize] % l) * l);
        (0..l as usize).for_each(|i| nums[i] /= l);
        nums
    }
}
// @lc code=end

