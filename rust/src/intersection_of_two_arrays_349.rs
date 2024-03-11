use crate::Solution;
use std::collections::HashSet;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1: HashSet<i32> = nums1.into_iter().collect();
        let mut nums2: HashSet<i32> = nums2.into_iter().collect();
        nums1.intersection(&nums2).cloned().collect()
    }
}
