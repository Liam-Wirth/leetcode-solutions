use crate::Solution;
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut out: i32 = 0;
        let s = s.trim().to_string();
        for char in s.chars().rev() {
            if char == ' ' {
                return out;
            } else {
                out += 1;
            }
        }
        out
    }
}
