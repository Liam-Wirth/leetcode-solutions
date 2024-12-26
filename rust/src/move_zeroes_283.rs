use crate::Solution;
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut pos = 0;  // where to place the next non-zero
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[pos] = nums[i];
                pos += 1;
            }
        }
        // fill the remainder with zeros
        while pos < nums.len() {
            nums[pos] = 0;
            pos += 1;
        }
    }
}

