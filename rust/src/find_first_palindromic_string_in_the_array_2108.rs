// NOTE: Solved in 7 minutes, would have been solved in like 3 if I didn't have trouble fighting
// rust lang being rust lang, something important to remember, as in an interview that could be bad
use crate::Solution;
impl Solution {
    pub fn first_palindrome(mut words: Vec<String>) -> String {
        for word in words.iter_mut() {
               if word == word.chars().rev().collect::<String>().as_str() {
                return word.clone()
            }
        }
        "".to_string()
    }

}
