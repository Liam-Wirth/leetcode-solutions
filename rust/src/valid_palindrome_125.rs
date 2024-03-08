use crate::Solution;
//NOTE: Double ended iterators are pretty awesome
impl Solution {
    pub fn is_palindrome_idiomatic(mut s: String) -> bool {
        let mut s = Self::clean_string(s.as_str());
        let rev: String = s.chars().rev().collect();
        rev == s
    }
    pub fn is_palindrome(s: String) -> bool {
        let mut s_chars = s
            .chars()
            .filter(|&ch| ch.is_alphanumeric())
            .map(|ch| ch.to_ascii_lowercase());

        while let Some(front) = s_chars.next() {
            if let Some(back) = s_chars.next_back() {
                if front != back {
                    return false;
                }
            } else {
                return true;
            }
        }
        true
    }
    fn clean_string(s: &str) -> String {
        let mut result = String::new();
        for c in s.chars() {
            if c.is_alphanumeric() {
                result.push(c.to_ascii_lowercase());
            }
        }
        result
    }
}
