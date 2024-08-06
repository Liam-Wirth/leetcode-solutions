use crate::Solution;

use std::collections::HashSet;

impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let set: HashSet<_> = nums.iter().collect();
        let mut ans = nums[0];

        for w in nums.windows(2) {
            if w[0] == w[1] - 1 {
                ans += w[1];
            } else {
                break;
            };
        }

        loop {
            if set.contains(&ans) {
                ans += 1;
                continue;
            }
            break ans;
        }
    }
}
