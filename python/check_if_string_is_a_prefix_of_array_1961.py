from typing import List

class Solution:
    def isPrefixString(self, s: str, words: List[str]) -> bool:
        prefix = ""
        for word in words:
            prefix += word
            if prefix == s:
                return True
            if len(prefix) > len(s):  # Early exit if the prefix is longer than s
                return False
        return False

