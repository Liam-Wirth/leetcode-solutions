use crate::Solution;
/*
 * Given two strings text1 and text2, return the length of their longest common subsequence. If there is no common subsequence, return 0.

A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative 
order of the remaining characters.

For example, "ace" is a subsequence of "abcde".

A common subsequence of two strings is a subsequence that is common to both strings.


Constraints:
1 <= text1.length, text2.length <= 1000
text1 and text2 consist of only lowercase English characters
*/

//NOTE: Dynamic programming is my OPP!!


//Thought Process:
//Rust is Really good with string stuff, and I'm pretty sure I can just like have
impl Solution {
    //text1 = ABCDE
    //text2 = ACE
    //two loop solution

    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut out: i32 = 0;
        let mut text1_vec = text1.chars().collect::<Vec<_>>();
        let mut text2_vec = text2.chars().collect::<Vec<_>>();
        let mut dp = vec![vec![0; text2.len()+1]; text1.len() + 1]; //look at excel sheet!
        println!("{:?}",dp);
        //O(n^2)
        for outer in 1..text1_vec.len() {
            for inner in 1..text2_vec.len() {
                if text1_vec[outer-1] == text2_vec[inner-1] {
                    dp[outer][inner] = dp[outer-1][inner-1]+1;
                } else {
                    dp[outer][inner] = <_>::max(dp[outer - 1][inner], dp[outer][inner - 1]);

                    println!("{}", dp[outer][inner]);
                }
            }
            println!("{:?}",dp);
        }
        //Return
        out = dp[text1_vec.len()-1][text2_vec.len()-1];
        println!("{}", out);
        out
    }
}
