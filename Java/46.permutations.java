package Java;
import java.util.ArrayList;
import java.util.List;

// @lc code=start
//TODO there is likely a good way to do this using hashmaps
class Solution {
    private void recurPermute(int[] nums, List<Integer> ds, List<List<Integer>> ans, boolean []freq) {
        if(ds.size()==nums.length){
            ans.add(new ArrayList<>(ds));
            return;
        }
        for(int i = 0; i<nums.length; i++) {
            if(!freq[i]) {
            freq[i] = true;
            ds.add(nums[i]);
            recurPermute(nums, ds, ans, freq);
            ds.remove(ds.size()-1);
            freq[i] = false;
            }
        }
    }

    public List<List<Integer>> permute(int[] nums) {
        List<List<Integer>> ans = new ArrayList<>();
        List<Integer> ds = new ArrayList<>();
        boolean freq[] = new boolean [nums.length];
        recurPermute(nums, ds, ans, freq);
        return ans;
    }

}
// @lc code=end

