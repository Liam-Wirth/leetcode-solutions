import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map.Entry;

// @lc code=start
class Solution {
    public boolean hasGroupsSizeX(int[] deck) {
      HashMap<Integer, Integer> numToFreq = new HashMap<>();
     
     for (int card: deck) {
         numToFreq.putIfAbsent(card, 0);
         numToFreq.put(card, numToFreq.get(card) + 1);
     }
     
     int groupSize = -1;
     for(Entry<Integer, Integer> elem : numToFreq.entrySet()) {
         if (groupSize == -1) {
             groupSize = elem.getValue();
         } else {
             groupSize = gcd(groupSize, elem.getValue());
         }
     }
     
     return groupSize >= 2;
 }
 
 public int gcd(int x, int y) {
     return x == 0 ? y : gcd(y%x, x);
 }
}

