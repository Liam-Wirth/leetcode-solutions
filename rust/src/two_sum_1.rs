use std::collections::HashMap; 
use crate::Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            let targ = target - num;
            if let Some(&index1) = hm.get(&targ) {
                return vec![index as i32, index1 as i32];
            }
            hm.insert(num, index);
        }
        return vec![0,0]
    
    }
    /* Old Solution {
        //we loooooove type inference
        let mut out = Vec::new();
//O(n^2) solution, maybe not the most optimal
        for (index1, num1) in nums.iter().enumerate() {
            for (index2, num2) in nums.iter().enumerate().skip((index1+1) as usize) {
               if  (num1 + num2 ) as i32 == target {
                   out.push(index1 as i32);
                   out.push(index2 as i32);
                   return out;
               } 
            }
        }
        vec![0,0]
       } */
}
