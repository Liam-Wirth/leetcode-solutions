use crate::Solution;

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let s_bytes = s.as_bytes();
        Self::helper(&s_bytes, 0, s_bytes.len() - 1)
    }

    fn helper(s: &[u8], start: usize, end: usize) -> i32 {
        if start == end {
            return 1;
        }
        if s[start] != s[end] {
            return (end - start + 1) as i32;
        } else {
            let mut new_start = start;
            let mut new_end = end;
            while new_start < new_end && s[new_start] == s[new_start + 1] {
                new_start += 1;
            }
            while new_start < new_end && s[new_end] == s[new_end - 1] {
                new_end -= 1;
            }
            if new_start == new_end {
                return 0;
            }
            return Self::helper(s, new_start + 1, new_end - 1);
        }
    }
}
