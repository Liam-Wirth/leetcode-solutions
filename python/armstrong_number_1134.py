class Solution:
    def isArmstrong(self, n: int) -> bool:
        s = str(n)
        check: int = 0
        k = len(s)
            
        for i in range(0, len(s)):
            check += int(s[i]) ** k
        return check == n

