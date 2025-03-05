use crate::Solution;
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
       let mut n = n as usize;
       let mut out: Vec<Vec<i32>> = vec![vec![0;n]; n];
       let mut count = 0;
       let (mut t, mut b, mut l,mut r) = (0, n-1, 0, n-1);

       while t <= b && l <= r {
        for i in l..=r {
          count += 1;
          out[t][i] = count;
        }
        t += 1;

        for i in t..=b {
          count += 1;
          out[i][r] = count;
        }
        r-=1;
        if r== std::usize::MAX {
          break;
        }
        for i in (l..=r).rev() {
          if t > b {
            continue;
          }
          count += 1;
          out[b][i] = count;
        }
        b -= 1;

        if b == std::usize::MAX {
          break;
        }
        for i in (t..=b).rev() {
          if l > r {
            continue;
          }
          count += 1;
          out[i][l] = count;
        }
        l+= 1;
       }
       out.into()
    }
}
