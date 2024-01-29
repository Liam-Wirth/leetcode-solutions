//TODO do more rust shit!
//Update as of Jan 8 2024, I'M TRYING!!!
use crate::Solution;
impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
       let mut y = 0;
       let z = x;
       while x>0{
        y *= 10;
        y += (x%10);
        x /= 10;
       }
       if y==z{
        true
       }
       else {
        false
       }
    }
}
// @lc code=end

