memo = {}
class Solution:
    def fib(self, n: int) -> int:
      if n in memo:
        return memo[n]
      if n <= 2:
        if n == 0:
          return 0
        result = 1
      else:
        result = self.fib(n-1) + self.fib(n-2)
      return result
