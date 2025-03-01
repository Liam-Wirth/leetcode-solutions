class Solution:
    def kthFactor(self, n: int, k: int) -> int:
      # two pass solution, find all the small factors first, then all the big factors
      for i in range(1, int(sqrt(n))+1):
        if n%i == 0:
          k -=1
          if k == 0:
            return i
      for i in range(int(sqrt(n)), 0, -1): # decrement from sqrt n to find all the big factors:
        if i*i == n: continue
        if n%i == 0:
          k-=1
          temp = n//i
          if k == 0:
            return temp
      return -1

