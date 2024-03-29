use crate::Solution;
use std::collections::HashSet;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
            let mut hs = HashSet::new();

            for i in nums.iter() {
                if hs.contains(i) {
                    return true;
                } else {hs.insert(i);}
            }
            false
    }
}
