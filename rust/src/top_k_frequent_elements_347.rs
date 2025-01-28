use std::collections::HashMap;
use crate::Solution;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        let mut out = Vec::new();
        for i in nums {
            *hm.entry(i).or_insert(0) += 1;
        }
        let mut temp: Vec<(&i32, &i32)> = hm.iter().collect();
        temp.sort_by(|a, b| b.1.cmp(a.1));
        for val in temp {
            if k > 0 {
                out.push(*val.0);
                k -= 1
            } else {
                break;
            }
        }
        out
    }
}
