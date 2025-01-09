use crate::Solution;
impl Solution {
    pub fn number_game(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        let mut result = Vec::new();

        for pair in nums.chunks(2) {
            if let [a, b] = pair {
                result.push(*b); // Bob appends
                result.push(*a); // Alice appends
            }
        }
        result
    }
}
