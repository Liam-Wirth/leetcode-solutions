use crate::Solution;
impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return 0;
        }
        nums.sort();

        let mut prev = 0;
        let mut out = 0;
        for n in nums {
            out += 0.max(prev - n);
            prev = n.max(prev) + 1;
            println!("{prev},{out}");
        }
        out
    }
}
