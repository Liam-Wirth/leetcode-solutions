from collections import Counter
class Solution: 
    def wordSubsets(self, words1: List[str], words2: List[str]) -> List[str]:
        out: list[str] = []
        maxf = Counter()

        for b in words2:
            freq_b = Counter(b)
            for ch, count in freq_b.items():
                maxf[ch] = max(maxf[ch], count)
        for a in words1:
            freq_a = Counter(a)
            if all(freq_a[ch] >= maxf[ch] for ch in maxf):
                out.append(a)
        return out

