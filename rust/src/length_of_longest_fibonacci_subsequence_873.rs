use std::collections::HashSet;

// TODO: Revisit
use crate::Solution;

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let nums: HashSet<i32> = arr.iter().cloned().collect();
        let mut maxlen = 0;

        for start in 0..arr.len() {
            for next in start + 1..arr.len() {
                let mut prev = arr[next];
                let mut curr = arr[start] + prev;
                let mut len = 2;
                while nums.contains(&curr) {
                    let temp = curr;
                    curr += prev;
                    prev = temp;
                    len += 1;
                    maxlen = maxlen.max(len);
                }
            }
        }

        maxlen
    }
}
