//Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such
//that each unique element appears only once. The relative order of the elements should be kept the same.
//Since it is impossible to change the length of the array in some languages, you must instead have 
//the result be placed in the first part of the array nums. More formally, if there are k elements after 
//removing the duplicates, then the first k elements of nums should hold the final result. It does not matter 
//what you leave beyond the first k elements.
//Return k after placing the final result in the first k slots of nums.
//NOTE:Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.
//TODO: solve this one with a two pointer solution on my very own without looking at this one
class Solution {

    public int removeDuplicates(int[] nums){
        int n = nums.length;
       //i could TOTALLY throw a hashmap at this >:)
        int left = 0;
        int right = 1;
        while(right<=n-1){
            if(nums[right]==nums[left]){
                right++;
            }else{
                nums[left+1] = nums[right];
                left++; right++;
            }

        }
        return left+1;
    }
}

