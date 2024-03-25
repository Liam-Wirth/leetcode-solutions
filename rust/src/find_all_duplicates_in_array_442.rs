use crate::Solution;
use std::collections::HashSet;
impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut hs = HashSet::new();
        let mut out: Vec<i32> = Vec::new();
        for num in nums {
            if hs.contains(&num){
                out.push(num);
            } else {
                hs.insert(num);
            }
        }
        out
    }
}
