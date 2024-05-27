use crate::Solution;
impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let mut nums = nums.clone();
        nums.sort();

        let n = nums.len();
        for x in 0..=n {
            let mut left = 0;
            let mut right = n as i32 - 1;

            while left <= right {
                let mid = (left + right) / 2;
                if nums[mid as usize] >= x as i32 {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }

            if n as i32 - left == x as i32 {
                return x as i32;
            }
        }

        -1
    }
}
