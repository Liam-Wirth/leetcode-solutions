use crate::Solution;

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let n = s.len();
        if n % 2 != 0 {
            return false;
        }

        // Convert s and locked to byte arrays for faster access
        let s: Vec<char> = s.chars().collect();
        let locked: Vec<char> = locked.chars().collect();

        let mut open_count = 0; // Tracks open '(' count
        let mut close_count = 0; // Tracks close ')' count
        let mut free_count = 0; // Tracks free slots (locked[i] == '0')

        // Left-to-right pass
        for i in 0..n {
            if locked[i] == '0' {
                free_count += 1;
            } else if s[i] == '(' {
                open_count += 1;
            } else {
                close_count += 1;
            }

            // Check if there are more ')' than '(' + free slots
            if close_count > open_count + free_count {
                return false;
            }
        }

        // Reset counts for right-to-left pass
        open_count = 0;
        close_count = 0;
        free_count = 0;

        // Right-to-left pass
        for i in (0..n).rev() {
            if locked[i] == '0' {
                free_count += 1;
            } else if s[i] == '(' {
                open_count += 1;
            } else {
                close_count += 1;
            }

            // Check if there are more '(' than ')' + free slots
            if open_count > close_count + free_count {
                return false;
            }
        }

        true
    }
}

