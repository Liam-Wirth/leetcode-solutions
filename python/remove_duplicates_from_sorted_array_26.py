class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
      seen = set()
      j = 0
      for i in range(0, len(nums)):
        if nums[i] not in seen:
          seen.add(nums[i])
          nums[j] = nums[i]
          j += 1
      return j
