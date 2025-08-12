use crate::Solution;
impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let (mut num1, mut num2) = (0, 0);

        for x in 1..=n {
            if x % m == 0 {
                num2 += x; // Sum of numbers divisible by m
            } else {
                num1 += x; // Sum of numbers not divisible by m
            }
        }
        num1 - num2
    }
}
