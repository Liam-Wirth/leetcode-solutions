import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;
import java.util.Stack;


// @lc code=start
class Solution {
    public boolean isValid(String s) {
     //NVM I changed my mind, I want to solve this problem a different way
     Stack<Character> stack = new Stack<>();
        for (char ch: s.toCharArray()) {
                if(ch == '(') stack.push(')');
                else if(ch == '{') stack.push('}');
                else if (ch == '[')stack.push(']');
                else if (stack.isEmpty() || stack.pop() != ch){
                return false;
            }
        }
    return stack.isEmpty();
    }
}
// @lc code=end

