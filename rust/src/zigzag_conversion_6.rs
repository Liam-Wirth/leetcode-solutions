use std::usize;

use crate::Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut cur: i32 = 0;
        let mut inc = 1;

        let mut res = vec![String::new(); num_rows as usize];
        for c in s.chars() {
            res[cur as usize].push(c);
            if cur == 0 {
                inc = 1;
            } else if cur == num_rows - 1 {
                inc = -1;
            }
            cur += inc;
        }

        let mut out = String::new();
        for r in res {
            out.push_str(&r);
        }
        out
    }
}

