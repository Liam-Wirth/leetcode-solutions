#typed: true
require 'sorbet-runtime'
# @lc code=start
# @param {Integer} x
# @return {Boolean}
def is_palindrome(x)
   y = 0
   z = x
   while x > 0 do
    y = y*10
    y = y+(x%10)
    x=x/10
   end
   if z==y
    return true
   else
    return false
   end

end
# @lc code=end

