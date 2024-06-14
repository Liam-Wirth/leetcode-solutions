use crate::Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut out = 0;
        let mut prev = 0;
        for n in nums{
            out += 0.max(prev-n +1);
            prev = n.max(prev+1);
        }
        out
    }
}
