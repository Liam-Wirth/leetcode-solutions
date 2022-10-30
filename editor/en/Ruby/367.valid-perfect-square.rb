# typed: true
# @lc app=leetcode id=367 lang=ruby
#
# [367] Valid Perfect Square
#
# https://leetcode.com/problems/valid-perfect-square/description/
#
# algorithms
# Easy (43.09%)
# Likes:    2415
# Dislikes: 239
# Total Accepted:    379.3K
# Total Submissions: 879.7K
# Testcase Example:  '16'
#
# Given a positive integer num, write a function which returns True if num is a
# perfect square else False.
#
# Follow up: Do not use any built-in library function such as sqrt.
#
#
# Example 1:
# Input: num = 16
# Output: true
# Example 2:
# Input: num = 14
# Output: false
#
#
# Constraints:
#
#
# 1 <= num <= 2^31 - 1
#
#
#

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

