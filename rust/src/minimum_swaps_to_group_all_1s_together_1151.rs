use crate::Solution;
impl Solution {
    pub fn min_swaps(data: Vec<i32>) -> i32 {
        let total: i32 = data.iter().sum();
        let mut l = 0;
        let mut r = 0;

        let mut best = 0;
        let mut cur = 0;

        while r < data.len() {
            cur += data[r];
            r += 1;
            
            if r - l > total  as usize{
                cur -= data[l];
                l += 1;
            }

            best = best.max(cur);
        }
        total - best
    }
}

