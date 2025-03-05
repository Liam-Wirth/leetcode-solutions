use crate::Solution;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
       // Doesn't matter what's in between the letters, just as long as they exist
       // in the sequence, and they are in the right order 

      let mut curr: usize = 0;
      let mut sarr: Vec<char> = s.chars().collect();

      if s.len() == 0 {
        return true;
      } 
      if t.len() == 0 || t.len() < s.len() {
        return false;
      }

      for c in t.chars() {
        if c == sarr[curr] {
          curr += 1;
          if curr ==  s.len() {
            return true;
          }
        }
      }
      false
    }
}
