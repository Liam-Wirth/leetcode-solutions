use std::collections::HashMap;
use crate::Solution;
// NOTE: Could most definitely optimize this

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut hm = HashMap::new();
        let mut out: i32 = 0;
        for num in nums.iter() {
            *hm.entry(num).or_insert(0) += 1;
        }
        for (k, v) in hm.iter() {
          if v > &(((nums.len() as i32)/2)) {
              return **k
          }  
        }
        out
    }
}
