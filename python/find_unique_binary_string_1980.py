class Solution:
    def findDifferentBinaryString(self, nums: List[str]) -> str:
        n = len(nums)
        res = ['0'] * n
        for i in range(n):
            res[i] = '1' if nums[i][i] == '0' else '0'
        return ''.join(res)
