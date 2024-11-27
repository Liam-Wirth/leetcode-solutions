class Solution:
    def specialArray(self, nums: List[int]) -> int:
        nums.sort()
        n = len(nums)

        for x in range(n + 1):
            # Binary search to find the first index where nums[index] >= x
            left, right = 0, n - 1
            while left <= right:
                mid = (left + right) // 2
                if nums[mid] >= x:
                    right = mid - 1
                else:
                    left = mid + 1

            # After the loop, left is the first index where nums[left] >= x
            if n - left == x:
                return x

        return -1

    def countGreaterThan(self, nums: List[int], num) -> int:
        i = self.binarySearch(nums, num)
        temp = 0
        if i == -1:
            for j in range(0, len(nums)):
                if nums[j] > num:
                    temp += 1
                    break
            print(num)
            return 1 + (len(nums) - temp)

        return len(nums) - i

    def binarySearch(self, nums: List[int], num) -> int:
        left = 0
        right = len(nums)

        while left < right:
            mid = int((left + right) / 2)
            if nums[mid] == num:
                return mid
            else:
                if nums[mid] < num:
                    left = mid + 1
                else:
                    right = mid - 1
        return -1
