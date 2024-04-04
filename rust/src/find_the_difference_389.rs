use crate::Solution;
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut ch: u8 = 0;
        for i in s.chars() {
            ch ^= i as u8;
        }
        for j in t.chars() {
            ch ^= j as u8;
        }
        ch as char
    }
}
