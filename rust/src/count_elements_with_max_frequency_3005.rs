use crate::Solution;
use std::collections::HashMap;
impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut max_val = 0;
        let mut hm: HashMap<_,_> = nums.iter().fold(HashMap::new(), |mut acc, &num| {
            let mut entry = acc.entry(num).or_insert(0);
            *entry += 1;
            max_val = max_val.max(*entry);
            acc
        });

          let out: i32 = hm.values()
            .filter(|&&freq| freq == max_val)
            .sum();

        out
    }
}
