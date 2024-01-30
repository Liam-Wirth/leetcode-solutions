
def roman_to_int(s)

# The ruby declaration for hashmaps is so pretty and understandable
hash = {
    'I' => 1,
    'V' => 5,
    'X' => 10,
    'L' => 50,
    'C' => 100,
    'D' => 500,
    'M' => 1000
}
  out = 0
  i = 0
while i < s.length
  if i + 1 < s.length && hash[s[i]] < hash[s[i + 1]]
    out += hash[s[i + 1]] - hash[s[i]]
        i+=1
     else
        out += hash[s[i]]
     end
    i += 1
   end
  out
end
# @lc code=end
