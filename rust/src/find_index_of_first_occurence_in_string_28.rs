use crate::Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map(|i| i as i32).unwrap_or(-1)
        //.find locates needle in the list and returns it as an option type
        //.map simply takes the value we get and makes it an i32
        //.unwrap_or just unwraps the option type, and returns -1 if it is None
    }
}
