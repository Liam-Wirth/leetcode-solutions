use crate::Solution;
impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
      let mut out: i64 = 1;
        for i in 0..n {
          out += 4*(i as i64);
        }
        out
    }
}

