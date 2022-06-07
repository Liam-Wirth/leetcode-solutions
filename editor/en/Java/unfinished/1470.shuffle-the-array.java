package Java;
import java.util.Arrays;

/*
 * @lc app=leetcode id=1470 lang=java
 *
 * [1470] Shuffle the Array
 *
 * https://leetcode.com/problems/shuffle-the-array/description/
 *
 * algorithms
 * Easy (88.38%)
 * Likes:    2412
 * Dislikes: 175
 * Total Accepted:    321.2K
 * Total Submissions: 363.4K
 * Testcase Example:  '[2,5,1,3,4,7]\n3'
 *
 * Given the array nums consisting of 2n elements in the form
 * [x1,x2,...,xn,y1,y2,...,yn].
 * 
 * Return the array in the form [x1,y1,x2,y2,...,xn,yn].
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: nums = [2,5,1,3,4,7], n = 3
 * Output: [2,3,5,4,1,7] 
 * Explanation: Since x1=2, x2=5, x3=1, y1=3, y2=4, y3=7 then the answer is
 * [2,3,5,4,1,7].
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: nums = [1,2,3,4,4,3,2,1], n = 4
 * Output: [1,4,2,3,3,2,4,1]
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: nums = [1,1,2,2], n = 2
 * Output: [1,2,1,2]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= n <= 500
 * nums.length == 2n
 * 1 <= nums[i] <= 10^3
 * 
 */

// @lc code=start
class Solution {
    public int[] shuffle(int[] nums, int n){
        //n = nums.length/2;
        int numsA[] = Arrays.copyOfRange(nums, 0, n-1);
        int numsB[] = Arrays.copyOfRange(nums, n-1, nums.length);
        int Sol[] = new int[(n*2)+1];
        for (int i = 0; i < nums.length-2; i++) {
            if(i%2 == 0){
                Sol[i]= numsA[i/2];
            }else{
                Sol[i]= numsB[i/2];
            }
        }
        return Sol;
    }     
    }
// @lc code=end

