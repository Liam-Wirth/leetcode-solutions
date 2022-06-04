import java.util.Arrays;
import java.util.HashMap;

/*
 * @lc app=leetcode id=169 lang=java
 *
 * [169] Majority Element
 *
 * https://leetcode.com/problems/majority-element/description/
 *
 * algorithms
 * Easy (63.10%)
 * Likes:    9916
 * Dislikes: 344
 * Total Accepted:    1.3M
 * Total Submissions: 2M
 * Testcase Example:  '[3,2,3]'
 *
 * Given an array nums of size n, return the majority element.
 * 
 * The majority element is the element that appears more than ⌊n / 2⌋ times.
 * You may assume that the majority element always exists in the array.
 * 
 * 
 * Example 1:
 * Input: nums = [3,2,3]
 * Output: 3
 * Example 2:
 * Input: nums = [2,2,1,1,1,2,2]
 * Output: 2
 * 
 * 
 * Constraints:
 * 
 * 
 * n == nums.length
 * 1 <= n <= 5 * 10^4
 * -10^9 <= nums[i] <= 10^9
 * 
 * 
 * 
 * Follow-up: Could you solve the problem in linear time and in O(1) space?
 */

// @lc code=start
class Solution {
    public int majorityElement(int[] nums) {

    Arrays.sort(nums);
    return nums[nums.length/2];
}
}
// @lc code=end

class alternateSolution{
    public int majorityElement(int[] nums){
      //when in doubt, throw a hashmap at it!
      HashMap<Integer, Integer> balls = new HashMap<>();
    int majority = nums.length/2;
      int cock = 0;
      //iterating the inputed array
      for (int i : nums) {
     //telling the hashmap to increment i's value if I exists
          balls.computeIfPresent(i, (key,val) -> val+1 );
          //if i doesn't exist, telling the hashmap to add i
          if(!balls.containsKey(i)) balls.put(i,1);
          //if i's value (it's count in the array) is greater than majority, return i;
          if(majority<balls.get(i).intValue()){
            cock = i;
            break;
          }

      }
      
      return cock;
    }
}

