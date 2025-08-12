class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        num_set = set(nums)
        out = 0

        for num in nums:
            if num - 1 not in num_set: #indicates start of a sequence
                cur = num
                curlen = 1

                while cur+1 in num_set:
                    cur = cur+1
                    curlen += 1
                if curlen > out:
                    out = curlen
        
        return out
