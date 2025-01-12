class Solution:
    def canBeValid(self, s: str, locked: str) -> bool:
        n = len(s)
        if n % 2 != 0:
            return False

        # Left-to-right pass
        open_count, close_count, free_count = 0, 0, 0
        for i in range(n):
            if locked[i] == '0':
                free_count += 1
            elif s[i] == '(':
                open_count += 1
            else:  # s[i] == ')'
                close_count += 1

            # Check if ')' exceeds '(' + free slots
            if close_count > open_count + free_count:
                return False

        # Right-to-left pass
        open_count, close_count, free_count = 0, 0, 0
        for i in range(n - 1, -1, -1):
            if locked[i] == '0':
                free_count += 1
            elif s[i] == '(':
                open_count += 1
            else:  # s[i] == ')'
                close_count += 1

            # Check if '(' exceeds ')' + free slots
            if open_count > close_count + free_count:
                return False

        return True

