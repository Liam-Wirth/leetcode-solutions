use crate::Solution;
impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = 2;

        let mut flips = 0;


        while l < nums.len() -2 {
            if nums[l] == 0 {
                flips += 1;
                for i in l..l+3 {
                    if nums[i] == 0 {
                        nums[i] = 1;
                    } else {
                        nums[i] = 0;
                    }
                }
            }
            if nums[l] == 1 {
                l += 1;
                r += 1;
            }
        }
        let sum: i32 = nums.iter().sum();

        if sum as usize == nums.len() {
            flips
        } else {
            -1
        }

        
    }
}

