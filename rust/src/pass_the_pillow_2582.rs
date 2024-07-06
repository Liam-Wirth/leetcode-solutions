use crate::Solution;

impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let chunks = time / (n - 1);
        if chunks % 2 == 0 {
            time % (n - 1) + 1
        } else {
            n - time % (n - 1)
        }
    }
}
