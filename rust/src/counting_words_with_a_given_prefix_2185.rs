use crate::Solution;
impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let mut out = 0;
        for w in words {
            if w.starts_with(pref.as_str()) {
                out +=1;
            }

        }
        out
    }
}

