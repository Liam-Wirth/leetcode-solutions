use crate::Solution;
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut min = 0;
        let mut max = 0;

        for ch in s.chars() {
            match ch {
                '(' => {
                    min += 1;
                    max += 1;
                }
                ')' => {
                    min -= 1;
                    max -= 1;
                }
                _ => {
                    min -= 1;
                    max += 1;
                }
            }
            if max < 0 {
                //Indicates there are too many opening parenthesis then could ever be closed
                return false;
            }
            if min < 0 {
                min = 0; //The only time min could get less than 0 is if it becomes negative one, here we just add 1 again
                         // Basically indicating that the * represents a null char/nothing, no change
            }
        }
        min == 0
    }
}
