use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut s_to_t_mapping: HashMap<char, char> = HashMap::new();
        let mut t_to_s_mapping: HashMap<char, char> = HashMap::new();

        for (s_char, t_char) in s.chars().zip(t.chars()) {
            if let Some(mapped_t_char) = s_to_t_mapping.get(&s_char) {
                if *mapped_t_char != t_char {
                    return false;
                }
            } else {
                s_to_t_mapping.insert(s_char, t_char);
            }

            if let Some(mapped_s_char) = t_to_s_mapping.get(&t_char) {
                if *mapped_s_char != s_char {
                    return false;
                }
            } else {
                t_to_s_mapping.insert(t_char, s_char);
            }
        }

        true
    }
}
