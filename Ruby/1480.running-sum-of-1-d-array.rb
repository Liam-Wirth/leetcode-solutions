# @lc code=start
# @param {Integer[]} nums
# @return {Integer[]}
def running_sum(nums)
    balls = []
    sum = 0
    nums.each_with_index do |num, i|
        sum+=num
        balls << sum
    end
    return balls
end
# @lc code=end

