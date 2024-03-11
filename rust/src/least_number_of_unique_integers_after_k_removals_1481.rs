use std::collections::HashMap;
use crate::Solution;
impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut hm = HashMap::new();
        let mut nums: Vec<i32> = vec![0; arr.len()];
        
        for num in arr.iter() {
            *hm.entry(*num).or_insert(0) += 1;
            nums.insert(*num as usize, 0);
        }

        let mut k = k;
        nums.sort_by(|a, b| b.cmp(a));
        let mut n = k;
        while n> 0 {
            n -= nums.pop().unwrap();
            if n < 0 {
                break;
            }
        }
        nums.len() as i32
    }

}