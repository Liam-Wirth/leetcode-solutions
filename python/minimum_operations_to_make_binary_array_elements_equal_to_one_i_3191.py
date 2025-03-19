class Solution:
    def minOperations(self, nums: List[int]) -> int:
        l = 0
        r = 2
        flips = 0
        while l < len(nums) - 2:
            if nums[l] == 0:
                flips += 1
                for i in range(l, l + 3):
                    if nums[i] == 0:
                        nums[i] = 1
                    else:
                        nums[i] = 0
            if nums[l] == 1:
                l += 1
                r += 1
        if sum(nums) == len(nums):
             return flips
        else:
            return -1
