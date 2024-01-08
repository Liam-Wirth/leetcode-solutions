struct Solution {}
impl Solution {
    //NOTE: use a search algorithm to find the longest string of the same numbers, and then modify
    //the rest of the string to be monotone increasing.
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let mut s1 = 0;
        let mut s2 = 0;

        for c in s.chars() {
            if c == '1' {
                s1 += 1;
            } else {
                s2 = s1.min(s2 + 1);
            }
        }
        s2
    }
}

fn main() {}
