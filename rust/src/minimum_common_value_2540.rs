use crate::Solution;
use std::collections::HashSet;
impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut hs: HashSet<i32> = nums1.iter().fold(HashSet::new(), |mut hs, &num| {
            hs.insert(num);
            hs
        });
            for i in 0..nums2.len() {
                if hs.contains(&nums2[i]) {
                    return nums2[i]
                }
            }
        -1
    }
}
