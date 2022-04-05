package com.liam.leetcode;

public class TwoSum {
    public static void main(String[] args) {
        Solution solution = new TwoSum().new Solution();
    }

    //leetcode submit region begin(Prohibit modification and deletion)
    class Solution {
        public int[] twoSum(int[] nums, int target) {
            int[] s = new int[2];
            for (int i = 0; i < nums.length; i++) {
                for (int j = i + 1; j <= nums.length - 1; j++) {
                    if (nums[i] + nums[j] == target) {
                        s[0] = i;
                        s[1] = j;
                    }
                }
            }
            return s;
        }
    }
//leetcode submit region end(Prohibit modification and deletion)

}