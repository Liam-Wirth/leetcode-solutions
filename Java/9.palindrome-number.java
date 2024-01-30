// @lc code=start
class Solution {
    public boolean isPalindrome(int x) {
       int y = 0;
       int z = x;
       while(x>0){
        y *= 10;
        y = y+(x%10);
        x/=10;
       }
       if (y == z) {
        return true;
       } else {
        return false;
       }
    }
}
// @lc code=end

