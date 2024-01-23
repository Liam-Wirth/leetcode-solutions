//HACK: I shouldnt have to do this.
struct Solution{}
//NOTE: Dynamic programming, We have two possibilities going down from the I-th house
//NOTE: Caching and such and so forth
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut money: i32 = 0;
        //HACK: Most definitely could use closures, no need to complicate it
        for i in 0..nums.len(){
               let curr= Self::hypothetical_payday(&*nums, i);
            money += curr;
            println!("Current index {}, hypothetical payday: {}", i, curr);
            println!("Money :{}",money);
        }
        println!("{}",money);
        money
    }

    pub fn hypothetical_payday(nums: &[i32], start: usize) -> i32 {
    if start >= nums.len() -1{
        return 0;
    }

    let rob_profit = nums[start] + Self::hypothetical_payday(nums, start + 2);
    let skip_profit = Self::hypothetical_payday(nums, start+1);

    // Compare and return the maximum of rob_profit and skip_profit
    Self::compare(rob_profit, skip_profit, nums[start], nums[start+1])
}
    pub fn compare(a: i32, b: i32, a_val: i32, b_val: i32) -> i32 {
        if a > b {
            return a_val;
        } else {
            b_val
        }
    }
}
fn main() {
    unsafe {
        Solution::rob(Vec::from([2,3,2,1]));
    }
}
