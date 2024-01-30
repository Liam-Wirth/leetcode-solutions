# @lc code=start
# @param {Integer[]} nums
# @param {Integer} k
# @return {Integer}
def maximum_top(nums, k)
    nums = [5,4,1,5,2,4,5]
    k = 6
 temp = []
 i = 0;
 if(nums.length <= 1)
    return -1;
 else
    until i == k-1 do
        unless nums.length <=1
            break;
        else
            temp.add(nums.shift());
        end
    end
end
for i in temp do
    puts " #i"
    puts "fuck"
end
puts "this is running"
return temp.max();
end
# @lc code=end

