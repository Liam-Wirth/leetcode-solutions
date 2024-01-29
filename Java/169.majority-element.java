import java.util.Arrays;
import java.util.HashMap;



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

