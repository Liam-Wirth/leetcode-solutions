//NOTE: Dynamic programming, We have two possibilities going down from the I-th house
//NOTE: Caching and such and so forth
use crate::Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut money: i32 = 0;
        //HACK: Most definitely could use closures, no need to complicate it
        hypothetical_payday(nums, 0)
    }
    pub fn hypothetical_payday(nums: Vec<i32>, start: i32) -> i32 {
        //NOTE: You can skip values in an iterator
        //NOTE: You can also sum
        //nums.iter().skip(i+1).sum();
        //base case
        if start>= nums.len() {
            return 0
        }
        //if we rob the house:
        let rob_profit = nums.get_unchecked(start as i32) + Self::hypothetical_payday(nums, start+2);
        // or we choose to skip the house:
        let skip_profit = Self::hypothetical_payday(nums, start+1);
        println!("Rob Profit: {}", rob_profit);
        println!("Skip Profit: {}", skip_profit);
        println!("Starting index: {}", start);
        compare(rob_profit, skip_profit)
        
    }
    pub fn compare(a: i32, b: i32) -> i32 {
        if a > b {
            return a;
        } else {
            b
        }
    }
}
fn main() {
    Solution::rob(Vec::from([8,0,0,8]));
}
