use crate::Solution;
impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut tmp: Vec<i32> = s
            .chars()
            .map(|key| key.to_ascii_lowercase() as i32)
            .collect();

        let n = tmp.len();
        let mut out = 0;
        for i in 0..n - 1 {
            out += (tmp[i] - tmp[i + 1]).abs();
        }
        out
    }
}

