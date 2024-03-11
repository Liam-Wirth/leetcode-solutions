
use crate::Solution;

impl Solution {
    pub fn missing_number(mut nums: Vec<i32>) -> i32 {
         nums = nums.sorted();

         for i in 0..nums.len() - 1{
             if nums.get(i+1) !=  nums.get(i) + 1 {
                 return nums.get(i) + 1;
             }
         }
      nums.len()
    }
}
