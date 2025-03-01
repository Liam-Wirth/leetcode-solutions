class Solution:
    def convert(self, s: str, numRows: int) -> str:
      if numRows == 1:
        return s
      out = [''] * numRows
      # need to zigzag, duh
      count = 0
      inc = 1
      for i, c in enumerate(s):
       out[count] += c
       if count == numRows-1:
        inc = -1
       elif count == 0:
        inc = 1
       count += inc
      ret = ""
      return ret.join(out)

