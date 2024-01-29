// @lc code=start
use crate::Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        /*!
        !IMPORTANT this doesnt need to be mutable!
        !wtf is a turbofish
        */
        let values = s.chars().map(|x| match x {
            'I' =>  1,
            'V' =>  5,
            'X' =>  10,
            'L' =>  50,
            'C' =>  100,
            'D' =>  500,
            'M' =>  1000,
            _ => 0,
        }).collect::<Vec<i32>>();
            let mut sum = values[0];
        for i in 0..values.len() -1 {

            if values[i] >= values[i+1] {
                sum+= values[i+1];
            }else {
                sum += values[i+1] - (values[i]*2);
            }
        }
        sum
    }

}
// @lc code=end

