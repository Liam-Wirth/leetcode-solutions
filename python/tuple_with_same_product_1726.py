# TODO: Revisit
class Solution:
    def tupleSameProduct(self, nums: List[int]) -> int:
        ans = 0
        counts = {}

        for i in range(len(nums)):
            for j in range(i + 1, len(nums)):
                prod = nums[i] * nums[j]
                counts[prod] = counts.get(prod, 0) + 1
        for val in counts.values():
            ans += 4 * val * (val - 1)
        return ans

