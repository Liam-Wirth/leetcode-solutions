class Solution:
    def applyOperations(self, nums: List[int]) -> List[int]:
      for i in range(0, len(nums)-1):
        if nums[i] == nums[i+1]:
          nums[i] *=2
          nums[i+1] = 0
      
      zeroes = []
      out = []
      for i in range(0, len(nums)):
        if nums[i] == 0:
          zeroes.append(0)
        else:
          out.append(nums[i])
      
      return out + zeroes

        

