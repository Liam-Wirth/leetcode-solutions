# TODO: Revisit
class Solution:
    def maxScore(self, s: str) -> int:
        total_ones = s.count("1")
        left_zeros = 0
        max_score = 0

        # Iterate through the string up to the second-to-last character
        for i in range(len(s) - 1):
            if s[i] == "0":
                left_zeros += 1
            else:
                total_ones -= 1
            current_score = left_zeros + total_ones
            max_score = max(max_score, current_score)

        return max_score

