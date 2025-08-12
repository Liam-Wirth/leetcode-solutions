class Solution:
    def search(self, nums: List[int], target: int) -> int:
        
        # Must be O(logN)
        
        left = 0
        right = len(nums) -1
        
        
        while left <= right:
            mid = (left + right) // 2
            if nums[mid] == target:
                return mid
            # First I wanna determine what half of the array is sorted
            # first let's check and see if its the left half:
            if nums[left] == target:
                return left
            elif nums[right] == target:
                return right
            if nums[left] < nums[mid]:
                #then left is sorted
                # is it possible for our target to be in this range?
                if nums[left] <= target < nums[mid]:
                    # since I did an inclusive search, check if nums[left] is target
                    if nums[left] == target:
                        return left
                    #yes, search the left half
                    right = mid -1
                else:
                    #no, we gotta check the right half (next time)
                    left = mid + 1
            else:
                #then the right half is sorted
                # is it possible for target to be in that range?
                if nums[mid] < target <= nums[right]:
                    # since I did an inclusive check, we gotta see if nums[right]
                    if nums[right] == target:
                        return right
                    left = mid + 1
                else: 
                    right = mid -1
        return -1
                    
                
            
            

