import java.util.Collections;
//TODO: There is likely a better way to go about solving this.
/*
 * @lc app=leetcode id=4 lang=java
 *
 * [4] Median of Two Sorted Arrays
 *
 * https://leetcode.com/problems/median-of-two-sorted-arrays/description/
 *
 * algorithms
 * Hard (34.29%)
 * Likes:    16943
 * Dislikes: 2043
 * Total Accepted:    1.4M
 * Total Submissions: 4.1M
 * Testcase Example:  '[1,3]\n[2]'
 *
 * Given two sorted arrays nums1 and nums2 of size m and n respectively, return
 * the median of the two sorted arrays.
 *
 * The overall run time complexity should be O(log (m+n)).
 *
 *
 * Example 1:
 *
 *
 * Input: nums1 = [1,3], nums2 = [2]
 * Output: 2.00000
 * Explanation: merged array = [1,2,3] and median is 2.
 *
 *
 * Example 2:
 *
 *
 * Input: nums1 = [1,2], nums2 = [3,4]
 * Output: 2.50000
 * Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
 *
 *
 *
 * Constraints:
 *
 *
 * nums1.length == m
 * nums2.length == n
 * 0 <= m <= 1000
 * 0 <= n <= 1000
 * 1 <= m + n <= 2000
 * -10^6 <= nums1[i], nums2[i] <= 10^6
 *
 *
 */

import java.util.ArrayList;
import java.util.Arrays;
import java.util.stream.IntStream;

// @lc code=start
class Solution {

    public double findMedianSortedArrays(int[] nums1, int[] nums2) {


    }
}
// @lc code=end
class test {
 public static void main(String[] args) {
     Solution solution = new Solution();
     solution.findMedianSortedArrays( new int[] {1,3}, new int[] {2});
 }
}
class OldSolution {

    public double oldfindMedianSortedArrays(int[] nums1, int[] nums2) {

        ArrayList<Integer> merged = new ArrayList<>();
        for (int i = 0; i <= nums1.length-1; i++) {
            merged.add(nums1[i]);
        }
        for (int i = 0; i <= nums2.length-1; i++) {
            merged.add(nums2[i]);
        }
        Collections.sort(merged);
        for (Integer integer : merged) {
            System.out.println(integer);
        }

        int fuggle =  merged.size();
        if (fuggle %2 == 0) {
            int median1 = (fuggle/2) - 1;
            int median2 = (fuggle/2);
            return (double) (merged.get(median1) + merged.get(median2))/2;

        } else {
            return (double) merged.get(fuggle/2);

        }

    }
}
