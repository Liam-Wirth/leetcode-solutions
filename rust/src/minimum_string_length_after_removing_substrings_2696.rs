use crate::Solution;

impl Solution {
    pub fn min_length(mut s: String) -> i32 {
        fn helper(s: &mut String, substrs: &[&str]) -> String {
            for &substr in substrs {
                if let Some(pos) = s.find(substr) {
                    let mut new = s.clone();
                    new.replace_range(pos..pos + substr.len(), "");
                    return helper(&mut new, substrs);
                }
            }
            s.clone()
        }
        let substrs = vec!["AB", "CD"];
        helper(&mut s, &substrs).len() as i32
    }
}
