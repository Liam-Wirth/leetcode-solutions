use crate::Solution;

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut sum = 0;

        for i in 0..nums.len() {
            if i == 0 || nums[i - 1] < nums[i] {
                sum += nums[i];
            } else {
                sum = nums[i];
            }
            res = res.max(sum);
        }

        res
    }
}

