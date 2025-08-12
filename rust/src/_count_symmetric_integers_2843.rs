use crate::Solution;

impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let mut out = 0;
        for i in low..=high { // Inclusive range
            let tmp = i.to_string(); // Convert integer to string
            let n = tmp.len();
            if n % 2 == 0 { // Check if the number has an even number of digits
                let front = tmp.chars().take(n / 2).map(|c| c.to_digit(10).unwrap()).sum::<u32>();
                let back = tmp.chars().skip(n / 2).map(|c| c.to_digit(10).unwrap()).sum::<u32>();
                if front == back {
                    out += 1;
                }
            }
        }
        out
    }
}
