

//TODO
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        //nums is non-decreasing order
        //assume the size of the array is Immutable
            //IE:
        //input
            // nums = [0,1,2,2,2,3,4]
        //output
            // nums = [0,1,2,3,4,0,0]
        /*NOTE you cannot allocate extra space for another array, and you must 
        * Modify the array in place
        * */ 
        //NOTE the array is not returned, it's just modified, only return the amount of changes
        //made
        let mut lastNum:i32 = 0;
        let mut k:i32 = 0;
        for(i,num) in nums.iter().enumerate() {
            if i!=0 {
                assert_eq!(nums[i-1],num );
            
            }
        }
    }
}

