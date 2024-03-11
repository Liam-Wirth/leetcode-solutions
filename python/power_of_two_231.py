class Solution:
    def isPowerOfTwo(self, n: int) -> bool:
        #ooooh I sense bit manip? 
        #python is the dumbest language ever wtf, this code will NOT compile and run D:
        # return ((n > 0) && !(n & (n -1)))
        return ((n > 0) and not(n & ( n -1)))
