use crate::Solution;
impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let n = s.len();
        let mut shift_effect = vec![0; n + 1]; // One extra for easier boundary handling

        // Mark the start and end of each shift
        for shift in shifts {
            let start = shift[0] as usize;
            let end = shift[1] as usize;
            let direction = if shift[2] == 1 { 1 } else { -1 };
            shift_effect[start] += direction;
            if end + 1 < n {
                shift_effect[end + 1] -= direction;
            }
        }

        // Calculate the prefix sum to get the net shifts
        for i in 1..n {
            shift_effect[i] += shift_effect[i - 1];
        }

        // Apply the shifts to the string
        let mut result = s.chars().enumerate().map(|(i, c)| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' } as i32;
                let shifted_char = (((c as i32 - base + shift_effect[i]) % 26 + 26) % 26 + base) as u8;
                shifted_char as char
            } else {
                c
            }
        }).collect::<String>();

        result
    }
}
