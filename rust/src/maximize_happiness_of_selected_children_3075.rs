use crate::Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        let mut out: i64 = 0;
        let mut heap = BinaryHeap::new();

        for i in happiness {
            heap.push(i);
        }

        let mut cur = 0;

        for i in 0..k {
            let mut temp = heap.pop();
            if temp.is_some() {
                let mut temp = temp.unwrap() - cur;
                if temp <= 0 {
                    temp = 0;
                }
                out += temp as i64;
                cur +=1;
            } else{
                return out;
            }
        }
        out
    }
}

