import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;

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

// @lc code=start
class Solution {
     private void recurPermute(int[] nums, StringBuilder ds, List<String> ans, boolean []freq) {
        if(ds.length()==nums.length){
            ans.add(ds.toString());
            return;
        }
        for(int i = 0; i<nums.length; i++) {
            if(!freq[i]) {
            freq[i] = true;
            ds.append(nums[i]);
            recurPermute(nums, ds, ans, freq);
            ds.deleteCharAt(ds.length()-1);
            freq[i] = false;
            }
        }
    }



    public String largestMultipleOfThree(int[] digits) {
        StringBuilder ds = new StringBuilder();
        Arrays.sort(digits);
        List<String> ans = new ArrayList<>();
        boolean freq[] = new boolean[digits.length];
        int[] tmp = digits;
        for (int i = tmp.length; i>0; i--) {
            tmp = Arrays.copyOf(tmp, i);
            recurPermute(tmp, ds, ans, freq);
        }
        List<String> multof3 = new ArrayList<String>();
            long max = 0;
      for (String str : ans) {
            long strNum = Long.parseLong(str);
            if(strNum%3 == 0){
                    max = Math.max(max,strNum);
            }
        }
        return Long.toString(max);
    }
    //TODO can I use Collections.swap here?
    private void swap(int arr[], int a, int b){
        int tmp = arr[a];
        arr[a] = arr[b];
        arr[b] = tmp;
    }
    public static void main(String[] args) {
        Solution solution = new Solution();
       String tmp = solution.largestMultipleOfThree(new int[] {8,6,7,1,0});
        System.out.println(tmp);
    }
}
// @lc code=end

