# TODO: Revisit this is just a good sliding window problem
class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
      n = len(s)
      maxlen = 0
      chars = set()
      left = 0

      # SLIDING WINDOW MOTHERFUCKAAAA
      for right in range(n):
        if s[right] not in chars:
          chars.add(s[right])
          maxlen = max(maxlen, right - left +1)
        else:
          #slide da window
          while s[right] in chars:
            chars.remove(s[left])
            left +=1
          chars.add(s[right]) # re-add the duplicate char, and start the window from there

      return maxlen

