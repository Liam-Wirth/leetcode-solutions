#typed: true
# @lc app=leetcode id=1647 lang=ruby
#TODO finish this one, is a little too advanced for me rn
# [1647] Minimum Deletions to Make Character Frequencies Unique
#
# https://leetcode.com/problems/minimum-deletions-to-make-character-frequencies-unique/description/
#
# algorithms
# Medium (56.45%)
# Likes:    2661
# Dislikes: 44
# Total Accepted:    142.8K
# Total Submissions: 240.6K
# Testcase Example:  '"aab"'
#
# A string s is called good if there are no two different characters in s that
# have the same frequency.
#
# Given a string s, return the minimum number of characters you need to delete
# to make s good.
#
# The frequency of a character in a string is the number of times it appears in
# the string. For example, in the string "aab", the frequency of 'a' is 2,
# while the frequency of 'b' is 1.
#
#
# Example 1:
#
#
# Input: s = "aab"
# Output: 0
# Explanation: s is already good.
#
#
# Example 2:
#
#
# Input: s = "aaabbbcc"
# Output: 2
# Explanation: You can delete two 'b's resulting in the good string "aaabcc".
# Another way it to delete one 'b' and one 'c' resulting in the good string
# "aaabbc".
#
# Example 3:
#
#
# Input: s = "ceabaacb"
# Output: 2
# Explanation: You can delete both 'c's resulting in the good string "eabaab".
# Note that we only care about characters that are still in the string at the
# end (i.e. frequency of 0 is ignored).
#
#
#
# Constraints:
#
#
# 1 <= s.length <= 10^5
# s contains only lowercase English letters.
#
#
#

# @lc code=start
# @param {String} s
# @return {Integer}
def min_deletions(s)
  freq = Hash.new();
s.each_char do |x|
    unless freq.has_key?(x)
      freq.store(x,1);
    else
      freq[x]+=1;
    end

  end
puts freq
tmp = freq.each_with_object({}) { |(k,v),g| (g[v] ||= []) << k }
puts tmp
  end

# @lc code=end

min_deletions("ceabaacb")
