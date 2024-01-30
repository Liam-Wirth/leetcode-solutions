

# @lc code=start
# @param {Integer[]} nums
# @param {Integer} n
# @return {Integer[]}
def shuffle(nums, n)
    x = nums.slice(0, n);
    y = nums.slice(n, n*2);
   x.zip(y).flatten.compact
end
# @lc code=end
puts shuffle((0..10).to_a, 5)

