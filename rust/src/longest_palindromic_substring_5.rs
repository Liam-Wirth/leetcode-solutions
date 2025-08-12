use crate::Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut res = String::from("");
        let mut reslen = 0;

        for i in 0..s.len() {
            // Odd length palindrome (centered at i)
            let mut l = i as i32; // Use i32 to handle potential negative index in expansion
            let mut r = i as i32;
            while l >= 0 && r < s.len() as i32 && s.chars().nth(l as usize) == s.chars().nth(r as usize) {
                if (r - l + 1) as usize > reslen {
                    res = s[l as usize..=r as usize].to_string();
                    reslen = (r - l + 1) as usize;
                }
                l -= 1;
                r += 1;
            }

            // Even length palindrome (centered between i and i+1)
            l = i as i32;
            r = (i + 1) as i32;
            while l >= 0 && r < s.len() as i32 && s.chars().nth(l as usize) == s.chars().nth(r as usize) {
                if (r - l + 1) as usize > reslen {
                    res = s[l as usize..=r as usize].to_string();
                    reslen = (r - l + 1) as usize;
                }
                l -= 1;
                r += 1;
            }
        }
        res
    }
}
