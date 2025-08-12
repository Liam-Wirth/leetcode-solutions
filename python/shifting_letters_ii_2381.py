from typing import List

class Solution:
    def shiftingLetters(self, s: str, shifts: List[List[int]]) -> str:
        n = len(s)
        shift_effect = [0] * (n + 1)  # One extra space for easier boundary handling

        # Mark the start and end of each shift
        for start, end, direction in shifts:
            shift_amount = 1 if direction == 1 else -1
            shift_effect[start] += shift_amount  # Apply shift at start
            shift_effect[end + 1] -= shift_amount  # Remove shift after the range

        # Calculate the net shift at each position using a prefix sum
        for i in range(1, n):
            shift_effect[i] += shift_effect[i - 1]

        # Apply the shifts to the string
        result = []
        for i, char in enumerate(s):
            base = ord('a') if char.islower() else ord('A')
            # Apply net shift and wrap around
            shifted_char = chr((ord(char) - base + shift_effect[i]) % 26 + base)
            result.append(shifted_char)
        return ''.join(result)

