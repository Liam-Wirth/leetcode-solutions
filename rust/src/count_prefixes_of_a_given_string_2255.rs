use crate::Solution;
impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        let mut out = 0;
        for w in words {
            if s.starts_with(w.as_str()) {
                out +=1;
            }
        }
        out
    }
}
