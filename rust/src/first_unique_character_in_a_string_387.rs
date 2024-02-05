use crate::Solution;
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut char_count = HashMap::new();
        let mut unique_chars = HashSet::new();

        for (i, ch) in s.chars().enumerate() {
            *char_count.entry(ch).or_insert(0) += 1;

            if char_count[&ch] == 1 {
                unique_chars.insert(ch);
            } else {
                unique_chars.remove(&ch);
            }
        }

        for (i, ch) in s.chars().enumerate() {
            if unique_chars.contains(&ch) {
                return i as i32;
            }
        }

        -1
    }
}
