class Solution:
    def differenceOfSum(self, nums: list[int]) -> int:
        element_sum = sum(nums)
        digit_sum = sum(int(digit) for num in nums for digit in str(num))
        return abs(element_sum - digit_sum)

