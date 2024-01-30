# @lc code=start
# @param {Integer} num
# @return {Boolean}
def is_perfect_square(num)
# My plan is to use the babylonian method
# and then check if the resulting approximation is a whole number
# first I'll get some base cases out of the way

if num == 0 || num == 1
    return true
   end
   i = 1
   result = 1
   while result < num
    result = i*i
    i+=1
   end
   if result==num
    return true
   else
    return false
   end
end
# @lc code=end

