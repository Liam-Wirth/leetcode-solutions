use crate::Solution;
impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut count = 0;
        let num_str = num.to_string();

        for digit_char in num_str.chars() {
            if let Some(digit) = digit_char.to_digit(10) {
                if digit != 0 && num % digit as i32 == 0 {
                    count += 1;
                }
            }
        }
        count
    }
}
