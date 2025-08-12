class Solution:
    def longestMonotonicSubarray(self, nums: List[int]) -> int:
        #wait I was gonna brute force but the constraints are chill
        # wait fuck it I'll do that anyway
        inc = 1
        dec = 1
        maxl = 1
        for pos in range(len(nums) -1):
            if nums[pos + 1] > nums[pos]:
                inc += 1
                dec =1 # reset
            elif nums[pos + 1] < nums[pos]:
                inc = 1 # reset
                dec += 1 
            else:
                inc = dec = 1
            maxl = max(inc, dec, maxl)
        return maxl
    

