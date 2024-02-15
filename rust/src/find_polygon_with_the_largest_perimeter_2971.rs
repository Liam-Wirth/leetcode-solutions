use crate::Solution;
impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        if nums.len() <=2 {
            return -1
        };
        nums.sort(); //perform in-place sort of the array
        
        let mut sum: i64 = 0;
        for num in nums.iter() {
            sum += *num as i64;
        }
        for num in nums.iter().rev() {
            sum -= *num as i64;
            if sum > (*num).into() {
                return sum + (*num as i64)
            }
        }
        
        -1
    }
}
