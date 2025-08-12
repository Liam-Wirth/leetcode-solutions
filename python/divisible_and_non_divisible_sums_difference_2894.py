class Solution:
    def differenceOfSums(self, n: int, m: int) -> int:
        num1 = sum(x for x in range(1, n + 1) if x % m != 0)
        num2 = sum(x for x in range(1, n + 1) if x % m == 0)
        return num1 - num2

