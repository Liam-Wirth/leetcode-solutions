class Solution:
    def removeDuplicates(self, nums: list[int]) -> int:
      seen = {}
      j = 0
      for i in range(0, len(nums)):
        if nums[i] not in seen:
          seen[nums[i]] = 1
          nums[j] = nums[i]
          j += 1
        elif seen[nums[i]] < 2:
            seen[nums[i]] += 1
            nums[j] = nums[i]
            j += 1
      return j
