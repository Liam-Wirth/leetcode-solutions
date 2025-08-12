use crate::Solution;

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut inc = 1;
        let mut dec = 1;
        let mut max = 1;

        for i in 0.. nums.len() - 1 {
            if nums[i+ 1] > nums[i] {
                inc +=1;
                dec = 1;

            }
            else if nums[i + 1] < nums[i] {
                inc = 1;
                dec +=1;

            }

            else { // they are equal :(
                inc = 1;
                dec = 1;
            }
            max = max.max(inc);
            max = max.max(dec);
        }
    max
    }
}
