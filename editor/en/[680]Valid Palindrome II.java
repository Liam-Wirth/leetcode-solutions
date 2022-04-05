
package com.liam.leetcode;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;

class ValidPalindromeII {
    public static void main(String[] args) {
        Solution solution = new ValidPalindromeII().new Solution();
        System.out.println(solution.validPalindrome("gfg"));
    }

    //leetcode submit region begin(Prohibit modification and deletion)
    class Solution {
        private boolean checkPalindrome(String s, int i, int j) {
            while (i < j) {
                if (s.charAt(i) != s.charAt(j)) return false;
                i++; j--;
            }

            return true;
        }

        public boolean validPalindrome(String s) {
            int i = 0, j = s.length() - 1;
            while (i < j) {
                if (s.charAt(i) != s.charAt(j)) {
                    return (checkPalindrome(s,i, j - 1) || checkPalindrome(s, i + 1, j));
                }
                    i++; j--;
            }
            return true;
        }
    }
//leetcode submit region end(Prohibit modification and deletion)
}
