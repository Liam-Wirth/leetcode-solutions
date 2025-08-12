class Solution:
    def countPalindromicSubsequence(self, s: str) -> int:
        count = 0
        lowercase_letters = 'abcdefghijklmnopqrstuvwxyz'
        for char in lowercase_letters:
            left = s.find(char)
            right = s.rfind(char)

            if right - left > 1:
                unique = set(s[left + 1: right])
                count += len(unique)
        return count


