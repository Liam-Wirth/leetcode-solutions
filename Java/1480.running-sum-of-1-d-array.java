package Java;


// @lc code=start
class Solution {
    public int[] runningSum(int[] nums) {
    int sol[] = new int[nums.length];
    sol[0] = nums[0];
        for (int i = 1; i < nums.length; i++) {
            sol[i] = sol[i-1]+nums[i];
        }
       return sol;
    }
}
// @lc code=end
class altSolution{
    public int[] altrunningSum(int[] nums) {
        int sol[] = new int[nums.length];
        int sum = 0;
            for (int i = 0; i < nums.length; i++) {
                sum+= nums[i];
                sol[i] = sum;
            }
           return sol;
        }
}
