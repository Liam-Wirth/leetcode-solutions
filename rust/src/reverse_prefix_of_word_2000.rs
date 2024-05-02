use crate::Solution;
impl Solution {
    pub fn reverse_prefix(mut word: String, ch: char) -> String {
        //process today: write very un-idiomatic code, then
        //do some research, and figure out how to write it
        // much more idiomatically

        if let Some(index) = word.find(ch) {
            let mut result = word[..=index].chars().rev().collect::<String>();
            result.push_str(&word[index + 1..]);
            result
        } else {
            word
        }
    }
}

