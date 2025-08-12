class Solution:
    def countPrefixSuffixPairs(self, words: List[str], ans = 0) -> int:

        for pre_suf, word in combinations(words,2):
            ans+= word.startswith(pre_suf) and word.endswith(pre_suf)

        return  ans

