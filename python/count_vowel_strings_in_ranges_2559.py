class Solution:
    def vowelStrings(self, words: List[str], queries: List[List[int]]) -> List[int]:
        vowels = {'a', 'e', 'i', 'o', 'u'}
        
        # Helper function to check if a word starts and ends with a vowel
        def is_vowel_string(word):
            return word[0] in vowels and word[-1] in vowels
        
        # Compute prefix sums
        n = len(words)
        prefix_sums = [0] * (n + 1)
        for i in range(n):
            prefix_sums[i + 1] = prefix_sums[i] + (1 if is_vowel_string(words[i]) else 0)
        
        # Answer each query
        results = []
        for li, ri in queries:
            count = prefix_sums[ri + 1] - prefix_sums[li]
            results.append(count)
        
        return results


