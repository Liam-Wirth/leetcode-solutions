class Solution:
    def findTargetSumWays(self, nums: List[int], target: int) -> int:
        dp = {}  # (index, total) -> # of ways

        def back(idx, tot):
            # always start with da basecase
            if idx == len(nums):
                return 1 if tot == target else 0
            if (idx, tot) in dp:
                return dp[(idx, tot)]

            dp[(idx, tot)] = back(idx + 1, tot + nums[idx]) + back(
                idx + 1, tot - nums[idx]
            )

            return dp[(idx, tot)]
        return back(0, 0)

