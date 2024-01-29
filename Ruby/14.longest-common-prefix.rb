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
