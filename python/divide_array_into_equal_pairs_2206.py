class Solution:
    def divideArray(self, nums: list[int]) -> bool:
        if len(nums)%2 == 1:
            return False
        seen = {}
        for num in nums:
            if num in seen:
                seen[num] +=1
            else:
                seen[num] = 1
        # print(seen)
        for (k, v)  in seen.items():
            if v % 2 == 1:
                return False
        return True


