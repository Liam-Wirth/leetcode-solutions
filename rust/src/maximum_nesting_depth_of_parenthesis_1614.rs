use crate::Solution;
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        //this is very simmilar to another problem I've solved, we are gonna use a stack
        let mut depth: Vec<char> = Vec::new();
        let mut max: usize = 0;
        //WE GREEDY IN THIS MOTHAFUCKA!
        if s.len() == 0 {
            return 0;
        }
        //else {      <--- commented out else block cause it's an old habbit and not sure if  I should keep doing it
        for ch in s.chars() {
            match (ch) {
                '(' => {
                    depth.push(ch);
                    if max < depth.len() {
                        max = depth.len();
                    }
                }
                ')' => {
                    depth.pop();
                }
                _ => {
                    continue; // we don't care!!
                }
            }
        }
        max as i32
    }
}
