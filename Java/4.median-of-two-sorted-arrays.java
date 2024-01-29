import java.util.Collections;
//TODO: There is likely a better way to go about solving this.
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
