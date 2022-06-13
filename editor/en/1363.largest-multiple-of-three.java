import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.stream.Collectors;

/*
 * @lc app=leetcode id=1363 lang=java
 *
 * [1363] Largest Multiple of Three
 *
 * https://leetcode.com/problems/largest-multiple-of-three/description/
 *
 * algorithms
 * Hard (33.99%)
 * Likes:    413
 * Dislikes: 56
 * Total Accepted:    14.2K
 * Total Submissions: 41.7K
 * Testcase Example:  '[8,1,9]'
 *
 * Given an array of digits digits, return the largest multiple of three that
 * can be formed by concatenating some of the given digits in any order. If
 * there is no answer return an empty string.
 *
 * Since the answer may not fit in an integer data type, return the answer as a
 * string. Note that the returning answer must not contain unnecessary leading
 * zeros.
 *
 *
 * Example 1:
 *
 *
 * Input: digits = [8,1,9]
 * Output: "981"
 *
 *
 * Example 2:
 *
 *
 * Input: digits = [8,6,7,1,0]
 * Output: "8760"
 *
 *
 * Example 3:
 *
 *
 * Input: digits = [1]
 * Output: ""
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= digits.length <= 10^4
 * 0 <= digits[i] <= 9
 *
 *
 */
//TODO redo this one and be able to solve it on my own, it was really really hard for me (as of June 12th 2022)
// @lc code=start
class Solution {
    public String largestMultipleOfThree(int[] digits) {
        if(digits.length == 1){
           if(digits[0]%3==0) return Integer.toString(digits[0]);

           else return "";

        }         int n = digits.length;
        int remainderOfSum = 0;
        //it is digits[i] can only be in range of 0 to 9
        int counter[] = new int[10];
        for (int i : digits ) {
            counter[i]++; //this keeps a count of the digits in my given array! so genius
            remainderOfSum += i;
        }
        int remainderOf1 = counter[1] + counter[4] + counter[7];
        int remainderOf2 = counter[2] + counter[5] + counter[8];

        remainderOfSum = remainderOfSum % 3;
        if(remainderOfSum == 1){
            if(remainderOf1 > 0){
                remainderOf1--;
            }
            else{
                remainderOf2 = remainderOf2 - 2;
            }
        }
        if(remainderOfSum == 2){
            if(remainderOf2 > 0){
                remainderOf2--;
            }
            else{
                remainderOf1 = remainderOf1 - 2;
            }
        }
        StringBuilder sb = new StringBuilder();
        for(int i = 9; i >= 0; i--){
            if(i % 3 == 0){
                while(counter[i] > 0){
                    sb.append(i);
                    counter[i]--;
                }
            }
            if(i % 3 == 1){
                while(counter[i] > 0 && remainderOf1 > 0){
                    sb.append(i);
                    counter[i]--;
                    remainderOf1--;
                }
            }
            if(i % 3 == 2){
                while(counter[i] > 0 && remainderOf2 > 0){
                    sb.append(i);
                    counter[i]--;
                    remainderOf2--;
                }
            }
        }//end for loop

        if(sb.length() == 0){
            return "";
        }
        else{
            if(sb.charAt(0) == '0'){
                return "0";
            }
            else{
                return sb.toString();
            }

        }

    }
}
