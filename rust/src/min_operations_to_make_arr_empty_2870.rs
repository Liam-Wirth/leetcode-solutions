use std::collections::HashMap;
use crate::Solution;
use std::collections::HashSet;
/* You are given a 0-indexed array nums consisting of positive integers.

There are two types of operations that you can apply on the array any number of times:

    Choose two elements with equal values and delete them from the array.
    Choose three elements with equal values and delete them from the array.

Return the minimum number of operations required to make the array empty, or -1 if it is not possible.
*/
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let mut total_operations = 0;
        for &num in nums.iter() {
            *count.entry(num).or_insert(0) += 1;
        }
        //NOTE: This solution uses the pigeonhole principle
        //pigeonhole principle of divisibility, basically, you want to group things into maximums
        //of three, and if they dont fit in 3, you might need just one more operation for that
        //extra group of 2, or 1
        for &freq in count.values() {
            if freq == 1 {
                return -1;
            }
                total_operations += freq / 3;
            if freq % 3 != 0 {
                total_operations+=1;
            } 
        }
        total_operations
    }
}
fn main() {
    unimplemented!();
}







