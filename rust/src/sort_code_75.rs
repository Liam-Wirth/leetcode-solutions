use crate::Solution;
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut i, mut i0, mut i2) = (0, 0, nums.len());
        
        while i < i2 {
            match nums[i] {
                0 => { nums.swap(i, i0); i0 += 1; i += 1; },
                2 => { i2 -= 1; nums.swap(i, i2); },
                _ => { i += 1 },
            }
        }
    }
}
