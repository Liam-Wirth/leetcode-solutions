class Solution:
    def minimumLength(self, s: str) -> int:
        return self.helper(s, 0, len(s) - 1)

    def helper(self, s: str, start: int, end: int) -> int:
        if start == end:
            return 1
        if s[start] != s[end]:
            return end - start + 1
        else:
            while start < end and s[start] == s[start + 1]:
                start += 1
            while start < end and s[end] == s[end - 1]:
                end -= 1
            if start == end:
                return 0
            return self.helper(s, start + 1, end - 1)
