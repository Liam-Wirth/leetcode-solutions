// NOTE: Unoptimized and slow asf, but we ball
use crate::Solution;
use std::collections::HashSet;
impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut hs = HashSet::new();
        let mut max = -1;
        for i in nums {
           hs.insert(i);
           if hs.contains(&(i*-1)){
            if max < i.abs() {
                max = i.abs();
            }
           }
        }
        max
    }
}
