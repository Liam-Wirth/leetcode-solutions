/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 *
 * https://leetcode.com/problems/palindrome-number/description/
 *
 * algorithms
 * Easy (52.49%)
 * Likes:    6050
 * Dislikes: 2163
 * Total Accepted:    2.2M
 * Total Submissions: 4.1M
 * Testcase Example:  '121'
 *
 * Given an integer x, return true if x is palindrome integer.
 *
 * An integer is a palindrome when it reads the same backward as forward.
 *
 *
 * For example, 121 is a palindrome while 123 is not.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: x = 121
 * Output: true
 * Explanation: 121 reads as 121 from left to right and from right to left.
 *
 *
 * Example 2:
 *
 *
 * Input: x = -121
 * Output: false
 * Explanation: From left to right, it reads -121. From right to left, it
 * becomes 121-. Therefore it is not a palindrome.
 *
 *
 * Example 3:
 *
 *
 * Input: x = 10
 * Output: false
 * Explanation: Reads 01 from right to left. Therefore it is not a
 * palindrome.
 *
 *
 *
 * Constraints:
 *
 *
 * -2^31 <= x <= 2^31 - 1
 *
 *
 *
 * Follow up: Could you solve it without converting the integer to a string?
 */

// @lc code=start
//TODO do more rust shit!
//Update as of Jan 8 2024, I'M TRYING!!!
use crate::Solution;
impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
       let mut y = 0;
       let z = x;
       while x>0{
        y *= 10;
        y += (x%10);
        x /= 10;
       }
       if y==z{
        true
       }
       else {
        false
       }
    }
}
// @lc code=end

