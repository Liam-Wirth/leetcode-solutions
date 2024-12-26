// TODO: Revist, not happy/understand the solution tbh
//REVIST
//NOTE:
use std::collections::HashMap;
use crate::Solution;


impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut pair_sums: HashMap<i32, Vec<(usize, usize)>> = HashMap::new();

        // pair_sums will hold sums of pairs with indices < c.
        // We'll fill pair_sums incrementally as c moves forward.

        for c in 0..nums.len() {
            // (1) Use whatever is in pair_sums so far to check for quadruplets
            //     with the chosen c and d in (c+1..n).
            for d in c + 1..nums.len() {
                let target = nums[d] - nums[c];
                if let Some(pairs) = pair_sums.get(&target) {
                    // Check only pairs (a,b) that satisfy b < c
                    for &(a, b) in pairs {
                        if b < c {
                            count += 1;
                        }
                    }
                }
            }

            // (2) Now insert all pairs that end exactly at c
            //     so they will be available for the *next* iteration's checks.
            for a in 0..c {
                let sum = nums[a] + nums[c];
                pair_sums.entry(sum).or_insert_with(Vec::new).push((a, c));
            }
        }

        count
    }
}

