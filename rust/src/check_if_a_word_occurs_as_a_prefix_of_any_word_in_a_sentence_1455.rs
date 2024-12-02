use crate::Solution;
impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let mut i: i32 = 1;
        for word in sentence.split_whitespace() {
            if word.starts_with(&search_word) {
                return i;
            }
            i += 1;
        }
        -1
    }
}
