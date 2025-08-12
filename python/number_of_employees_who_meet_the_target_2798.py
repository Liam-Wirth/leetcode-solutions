class Solution:
    def numberOfEmployeesWhoMetTarget(self, hours: List[int], target: int) -> int:
        out = 0
        for i in hours:
            if i >= target:
                out +=1
        return out

