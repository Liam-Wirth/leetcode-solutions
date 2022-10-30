import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;
import java.util.Stack;

/*
 * @lc app=leetcode id=20 lang=java
 *
 * [20] Valid Parentheses
 *
 * https://leetcode.com/problems/valid-parentheses/description/
 *
 * algorithms
 * Easy (40.86%)
 * Likes:    13558
 * Dislikes: 613
 * Total Accepted:    2.3M
 * Total Submissions: 5.7M
 * Testcase Example:  '"()"'
 *
 * Given a string s containing just the characters '(', ')', '{', '}', '[' and
 * ']', determine if the input string is valid.
 *
 * An input string is valid if:
 *
 *
 * Open brackets must be closed by the same type of brackets.
 * Open brackets must be closed in the correct order.
 * therefore (({()})) could also be valid
 * fuck
 * how do that
 *
 *
 * Example 1:
 *
 *
 * Input: s = "()"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: s = "()[]{}"
 * Output: true
 *
 *
 * Example 3:
 *
 *
 * Input: s = "(]"
 * Output: false
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 10^4
 * s consists of parentheses only '()[]{}'.
 *
 *
 */

// @lc code=start
class Solution {
    public boolean isValid(String s) {
     //NVM I changed my mind, I want to solve this problem a different way
     Stack<Character> stack = new Stack<>();
        for (char ch: s.toCharArray()) {
                if(ch == '(') stack.push(')');
                else if(ch == '{') stack.push('}');
                else if (ch == '[')stack.push(']');
                else if (stack.isEmpty() || stack.pop() != ch){
                return false;
            }
        }
    return stack.isEmpty();
    }
}
// @lc code=end

