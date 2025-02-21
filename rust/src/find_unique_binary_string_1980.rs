use crate::Solution;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums.len();
        let mut res = vec!['0'; n];
        for i in 0..n {
            res[i] = if nums[i].as_bytes()[i] == b'0' { '1' } else { '0' };
        }
        res.into_iter().collect()
    }
}
