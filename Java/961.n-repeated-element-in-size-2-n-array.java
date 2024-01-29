package Java;
import java.util.HashMap;

// @lc code=start
class Solution {
    public int repeatedNTimes(int[] nums) {
        int n = (nums.length-1)/2;
        int sol = 0;
        //when in doubt throw a hash map at it
        HashMap<Integer, Integer> ibo = new HashMap<>();
        for (int j : nums) {
            ibo.computeIfPresent(j, (key,val) -> val+1);
            if(!ibo.containsKey(j)){
                ibo.put(j,1);
            }
            if(ibo.get(j).intValue()>n){
                sol = j++;
            }
              
        }
        return sol;
            
        }

    }

// @lc code=end

