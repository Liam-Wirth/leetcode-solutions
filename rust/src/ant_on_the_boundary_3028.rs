use crate::Solution;
impl Solution {
    pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut cnt = 0;
        for i in nums {
            res += i;
            if res == 0 {
                cnt += 1;
            }
            
        }
        return cnt;
    }
}
