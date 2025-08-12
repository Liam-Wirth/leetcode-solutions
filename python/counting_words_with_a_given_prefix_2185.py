class Solution:
    def prefixCount(self, words: List[str], pref: str) -> int:
        out: int = 0
        for w in words:
            if w.startswith(pref): out+=1
        return out
        

