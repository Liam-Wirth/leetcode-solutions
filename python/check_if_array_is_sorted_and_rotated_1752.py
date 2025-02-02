class Solution:
    def check(self, nums: list[int]) -> bool:
        if nums == sorted(nums):
            return True
        else:
            rot_count = 0
            for i in range(0, len(nums)):
                if nums[i] > nums[(i+1) % len(nums)]:
                    rot_count+=1
                    if rot_count > 1:
                        return False
                    
        return True

                

        

