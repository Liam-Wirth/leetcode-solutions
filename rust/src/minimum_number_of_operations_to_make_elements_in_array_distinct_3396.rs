use crate::Solution;
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
       let mut seen = [false; 128];
       for i in (0..nums.len()).rev() {
        let num = nums[i] as usize;
        if seen[num] {
            return (i as i32)/ 3+1;
        }
        seen[num] = true;
       }
       0
    }
}

