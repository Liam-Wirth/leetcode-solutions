  
  package com.liam.leetcode;

  import java.util.ArrayList;
  import java.util.Arrays;
  import java.util.List;

  public class BinaryGap{
      public static void main(String[] args) {
           Solution solution = new BinaryGap().new Solution();
      }
      //leetcode submit region begin(Prohibit modification and deletion)
class Solution {

    public int binaryGap(int n) {
        String n1 = Integer.toBinaryString(n); //shitty unoptimized way to do it :/
                                    //TODO: LEARN BIT MANIPULATION!!!
        int maxdist = 0;
        //I made this negative one so that it misses the second
        //if statement that actually calculates the max down there V
        int last1 = -1;
        for (int i = 0; i <n1.length(); i++) {
            if (n1.charAt(i) == '1'){
                if (last1>=0){
                   maxdist = Math.max(maxdist,i-last1);
                }
                last1 = i;
            }

        }
        return maxdist;
    }
}
//leetcode submit region end(Prohibit modification and deletion)

  }