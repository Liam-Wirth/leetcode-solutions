use crate::Solution;
impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        // Initialize the sum
        let mut total_sum = 0;

        // Iterate through all numbers from 1 to n
        for x in 1..=n {
            // Check if the number is divisible by 3, 5, or 7
            if x % 3 == 0 || x % 5 == 0 || x % 7 == 0 {
                total_sum += x; // Add the number to the total sum
            }
        }

        total_sum // Return the computed sum
    }
}

