use crate::Solution;
impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        use std::collections::HashSet;
        let unique_chars: HashSet<char> = sentence.chars().collect();
        unique_chars.len() == 26
    }
}

