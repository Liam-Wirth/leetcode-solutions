use crate::Solution;
impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        if nums.len() <= 4 {
            return 0;
        }
        nums.sort();
        
        let mut dif = 2147483647;
        println!("num {:?}", nums);
        for i in 0..4 {
            let r = nums.len() -4 + i;
            dif = dif.min(nums[r] - nums[i]);
        }

        dif
    }
}
