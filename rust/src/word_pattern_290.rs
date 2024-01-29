/*
 * @lc app=leetcode id=290 lang=rust
 *
 * [290] Word Pattern
 *
 * https://leetcode.com/problems/word-pattern/description/
 *
 * algorithms
 * Easy (40.25%)
 * Likes:    3680
 * Dislikes: 432
 * Total Accepted:    371.4K
 * Total Submissions: 922.7K
 * Testcase Example:  '"abba"\n"dog cat cat dog"'
 *
 * Given a pattern and a string s, find if s follows the same pattern.
 *
 * Here follow means a full match, such that there is a bijection between a
 * letter in pattern and a non-empty word in s.
 *
 *
 * Example 1:
 *
 *
 * Input: pattern = "abba", s = "dog cat cat dog"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: pattern = "abba", s = "dog cat cat fish"
 * Output: false
 *
 *
 * Example 3:
 *
 *
 * Input: pattern = "aaaa", s = "dog cat cat dog"
 * Output: false
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= pattern.length <= 300
 * pattern contains only lower-case English letters.
 * 1 <= s.length <= 3000
 * s contains only lowercase English letters and spaces ' '.
 * s does not contain any leading or trailing spaces.
 * All the words in s are separated by a single space.
 *
 *
 */
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
