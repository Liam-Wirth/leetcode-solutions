package Java;
/*
 * @lc app=leetcode id=2202 lang=java
 *
 * [2202] Maximize the Topmost Element After K Moves
 *
 * https://leetcode.com/problems/maximize-the-topmost-element-after-k-moves/description/
 *
 * algorithms
 * Medium (22.16%)
 * Likes:    299
 * Dislikes: 205
 * Total Accepted:    16K
 * Total Submissions: 72.2K
 * Testcase Example:  '[5,2,2,4,0,6]\n4'
 *
 * You are given a 0-indexed integer array nums representing the contents of a
 * pile, where nums[0] is the topmost element of the pile.
 *
 * In one move, you can perform either of the following:
 *
 *
 * If the pile is not empty, remove the topmost element of the pile.
 * If there are one or more removed elements, add any one of them back onto the
 * pile. This element becomes the new topmost element.
 *
 *
 * You are also given an integer k, which denotes the total number of moves to
 * be made.
 *
 * Return the maximum value of the topmost element of the pile possible after
 * exactly k moves. In case it is not possible to obtain a non-empty pile after
 * k moves, return -1.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [5,2,2,4,0,6], k = 4
 * Output: 5
 * Explanation:
 * One of the ways we can end with 5 at the top of the pile after 4 moves is as
 * follows:
 * - Step 1: Remove the topmost element = 5. The pile becomes [2,2,4,0,6].
 * - Step 2: Remove the topmost element = 2. The pile becomes [2,4,0,6].
 * - Step 3: Remove the topmost element = 2. The pile becomes [4,0,6].
 * - Step 4: Add 5 back onto the pile. The pile becomes [5,4,0,6].
 * Note that this is not the only way to end with 5 at the top of the pile. It
 * can be shown that 5 is the largest answer possible after 4 moves.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [2], k = 1
 * Output: -1
 * Explanation:
 * In the first move, our only option is to pop the topmost element of the
 * pile.
 * Since it is not possible to obtain a non-empty pile after one move, we
 * return -1.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^5
 * 0 <= nums[i], k <= 10^9
 *
 *
 */
//TODO figure out what the hell is going on in this code
// @lc code=start
class Solution {
    public int maximumTop(int[] nums, int k) {
        int leng = nums.length, max = -1; //didn't know you could write them inline like that
        if(leng==1){
            if(k%2==1) return -1;
            else return nums[0];
        }
        if(k<leng) max = nums[k];
        else if(k>leng) k = leng+1;
        for (int i = 0; i < k-1; i++) {
            max = Math.max(max,nums[i]);

        }
       return max;
    }
}
// @lc code=end

