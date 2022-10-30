#typed: true
#
# @lc app=leetcode id=14 lang=ruby
#
# [14] Longest Common Prefix
#
# https://leetcode.com/problems/longest-common-prefix/description/
#
# algorithms
# Easy (39.82%)
# Likes:    8641
# Dislikes: 3129
# Total Accepted:    1.7M
# Total Submissions: 4.2M
# Testcase Example:  '["flower","flow","flight"]'
#
# Write a function to find the longest common prefix string amongst an array of
# strings.
#
# If there is no common prefix, return an empty string "".
#
#
# Example 1:
#
#
# Input: strs = ["flower","flow","flight"]
# Output: "fl"
#
#
# Example 2:
#
#
# Input: strs = ["dog","racecar","car"]
# Output: ""
# Explanation: There is no common prefix among the input strings.
#
#
#
# Constraints:
#
#
# 1 <= strs.length <= 200
# 0 <= strs[i].length <= 200
# strs[i] consists of only lowercase English letters.
#
#
#

# @lc code=start
# @param {String[]} strs
# @return {String}
def longest_common_prefix(strs)
    return "" if strs == []
    common_pre = ""
    short_str = (strs.sort!).delete_at(0)
    short_str.each_char do |c|
         strs.each do |str|
            if(str.start_with?(common_pre + c))
                next
            else
                return common_pre
            end
         end
        common_pre += c
    end
end
# @lc code=end
longest_common_prefix(["fug","fung","fuck"])
