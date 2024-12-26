# TODO: Revisit this cause it would be nice to try and either just remember the digital root thing, or be able to do the naive solution
class Solution:
    def addDigits(self, num: int) -> int:
        if num == 0:
            return 0
        else:
            return 1 + (num-1) % 9
        

