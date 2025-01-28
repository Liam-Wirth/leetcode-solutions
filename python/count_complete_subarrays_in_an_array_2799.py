# TODO: Revisit again, this is a good sliding window problem
class Solution:
    #example:
    """
    input = [1,3,1,2,2]
    distinct arrays include:
    [1,3,1,2,2]
    [3, 1, 2, 2]
    [3,1,2]
    [1,3,1,2]

    """
    def countCompleteSubarrays(self, nums: list[int]) -> int:
        n = len(nums)
        ans = 0
        left = 0
        seen = {}
        distinct = len(set(nums)) # list of distinct elements
        for right in range(n):
            # print(seen)
            seen[nums[right]] = seen.get(nums[right], 0) +1

            while len(seen) == distinct:
                ans += n - right

                # this is shifting the window
                seen[nums[left]] -= 1
                if seen[nums[left]] == 0:
                    del seen[nums[left]]
                left += 1 
        return ans






