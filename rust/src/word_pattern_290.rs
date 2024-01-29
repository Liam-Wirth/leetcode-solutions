use crate::Solution;
// @lc code=start
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let vec: Vec<&str> = s.split_whitespace().collect();
        let mut cw = HashMap::new();
        let mut i = 0;
        let mut ws = HashSet::new();
        if pattern.len() != vec.len() {
            return false;
        }
        for c in pattern.chars() {
            let w = vec[i];
            i += 1;
            if cw.contains_key(&c) {
                if cw[&c] != w {
                    return false;
                } else {
                    continue;
                }
            } else if ws.contains(w) {
                return false;
            }
            ws.insert(w.to_string());
            cw.insert(c, w.to_string());
        }
        true
    }
}
// @lc code=end
