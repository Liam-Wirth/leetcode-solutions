class Solution:
    def findMissingAndRepeatedValues(self, grid: List[List[int]]) -> List[int]:
      hs = set()
      n = len(grid)
        for i in range(0, (n*n)+1):
            hs.add(i)

      out = [0]*2
      for i in range(0, len(grid)):
        for j in range(0, len(grid[1])):
            num = grid[i][j]
            if num in hs:
                out[0] = num
            el
                
        

