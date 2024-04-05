use crate::Solution;
impl Solution {
    pub fn make_good(s: String) -> String {
        if s.len() == 1 {
            return s;
        }
        let mut out: Vec<char> = Vec::new();
        for ch in s.chars() {
            if let Some(&top) = out.last() {
                if top.eq_ignore_ascii_case(&ch) && top != ch {
                    out.pop();
                } else {
                    out.push(ch);
                }
            } else {
                out.push(ch);
            }
        }
        out.into_iter().collect::<String>()
    }
}
