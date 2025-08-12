use crate::Solution;
impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        //let mut out: i64 = 1;
        //  for i in 0..n {
        //    out += 4*(i as i64);
        //  }
        // Sum of first n natural numbers:
        let n: i64 = n as i64;
        let out: i64 = 1 + 4 * ((n - 1) * n) / 2;
        out
    }
}
