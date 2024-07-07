use crate::Solution;
impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        if num_exchange > num_bottles {
            return num_bottles;
        } else {
            return num_bottles + (num_bottles - 1) / (num_exchange - 1);
        }
    }
}
