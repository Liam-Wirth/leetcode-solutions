class Solution:
    def checkPowersOfThree(self, n: int) -> bool:
        def convert(i: int):
          if (i == 0):
            return ""
          x = i %3
          i = i // 3
          out = str(x)
          out += str(convert(i))

          return out
        tern = convert(n)
        if "2" in tern:
          return False
        return True


          

