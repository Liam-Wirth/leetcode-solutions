#typed: true
# @lc app=leetcode id=69 lang=ruby
#
# [69] Sqrt(x)
#
# https://leetcode.com/problems/sqrtx/description/
#
# algorithms
# Easy (36.60%)
# Likes:    4033
# Dislikes: 3269
# Total Accepted:    1.1M
# Total Submissions: 2.9M
# Testcase Example:  '4'
#
# Given a non-negative integer x, compute and return the square root of x.
#
# Since the return type is an integer, the decimal digits are truncated, and
# only the integer part of the result is returned.
#
# Note: You are not allowed to use any built-in exponent function or operator,
# such as pow(x, 0.5) or x ** 0.5.
#
#
# Example 1:
#
#
# Input: x = 4
# Output: 2
#
#
# Example 2:
#
#
# Input: x = 8
# Output: 2
# Explanation: The square root of 8 is 2.82842..., and since the decimal part
# is truncated, 2 is returned.
#
#
# Constraints:
#
#
# 0 <= x <= 2^31 - 1
#
#
#

# @lc code=start
# @param {Integer} x
# @return {Integer}
def my_sqrt(num)
    ans,y,e = num, 1, 0.01
    while ans - y > e
        ans = (ans+y)/2
        y = num/ans
    end
    ans.truncate()
end
# @lc code=end

