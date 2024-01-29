// @lc code=start
use crate::Solution;
impl Solution {
    //honestly I barely understand what's going on
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
       accounts.iter().map(|ac| ac.iter().sum()).max().unwrap()
    }
}
// @lc code=end

