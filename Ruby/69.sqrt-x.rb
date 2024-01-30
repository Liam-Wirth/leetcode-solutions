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

