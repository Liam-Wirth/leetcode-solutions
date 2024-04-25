// TODO: Revisit this!
impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let mut dp = vec![0; 26];
        for c in s.chars() {
            let i = (c as u8 - b'a') as usize; // Convert char to ASCII value
            let start = i.saturating_sub(k as usize).max(0); // Ensure start index is at least 0
            let end = (i + k as usize).min(25); // Ensure end index is at most 25
            let max_value = dp[start..=end].iter().max().copied().unwrap_or(0);
            dp[i] = max_value + 1;
        }
        dp.iter().max().copied().unwrap_or(0) // Return the maximum value in dp
    }
}

