use crate::Solution;
impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        // Calculate the element sum
        let element_sum: i32 = nums.iter().sum();
        
        // Calculate the digit sum
        let digit_sum: i32 = nums
            .iter()
            .flat_map(|&num| num.to_string().chars().collect::<Vec<char>>())
            .map(|digit| digit.to_digit(10).unwrap() as i32)
            .sum();
        
        // Return the absolute difference
        (element_sum - digit_sum).abs()
    }
}

