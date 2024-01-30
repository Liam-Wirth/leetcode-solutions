# @lc code=start
# @param {Integer[]} nums
# @return {Integer[]}
def build_array(nums)
cock = []
nums.each_with_index do |num, i|
  cock << nums[nums[i]]
end
return cock
end
# @lc code=end

