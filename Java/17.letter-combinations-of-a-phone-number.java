import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/*
 * @lc app=leetcode id=17 lang=java
 *
 * [17] Letter Combinations of a Phone Number
 *
 * https://leetcode.com/problems/letter-combinations-of-a-phone-number/description/
 *
 * algorithms
 * Medium (54.44%)
 * Likes:    10961
 * Dislikes: 707
 * Total Accepted:    1.2M
 * Total Submissions: 2.3M
 * Testcase Example:  '"23"'
 *
 * Given a string containing digits from 2-9 inclusive, return all possible
 * letter combinations that the number could represent. Return the answer in
 * any order.
 *
 * A mapping of digits to letters (just like on the telephone buttons) is given
 * below. Note that 1 does not map to any letters.
 *
 *
 * Example 1:
 *
 *
 * Input: digits = "23"
 * Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
 *
 *
 * Example 2:
 *
 *
 * Input: digits = ""
 * Output: []
 *
 *
 * Example 3:
 *
 *
 * Input: digits = "2"
 * Output: ["a","b","c"]
 *
 *
 *
 * Constraints:
 *
 *
 * 0 <= digits.length <= 4
 * digits[i] is a digit in the range ['2', '9'].
 *
 *
 */

// @lc code=start
class Solution {
    public List<String> letterCombinations(String digits) {
        String digitletter[] = {"","","abc","def","ghi","jkl","mno","pqrs","tuv","wxyz"};
            List<String> result = new ArrayList<String>();

            if (digits.length()==0) return result;

            result.add("");
            for (int i=0; i<digits.length(); i++)
                result = combine(digitletter[digits.charAt(i)-'0'],result);

            return result;
        }

        public List<String> combine(String digit, List<String> l) {
            List<String> result = new ArrayList<String>();

            for (int i=0; i<digit.length(); i++)
                for (String x : l)
                    result.add(x+digit.charAt(i));

            return result;
        }
    }
// @lc code=end

