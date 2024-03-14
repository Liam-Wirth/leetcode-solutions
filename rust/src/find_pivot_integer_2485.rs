use crate::Solution;
impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let mut arr:Vec<i32> = (1..n+1).collect();
        let mut temp  = 0;
        let mut sum:i32 = (1..n+1).sum();
        for i in 0..n {
            let j = arr.pop().unwrap();
            sum-=j;
            if temp == sum {
                return j
            } else {
                temp += j;
            }
        }
        
        -1
    }

    fn prefix_sum(arr: &[i32]) -> Vec<i32> {
        let mut prefix_sums = Vec::with_capacity(arr.len());
       arr.iter().fold(0, |acc, &x| {
          let sum = acc + x;

          prefix_sums.push(sum);
          sum
     });
    prefix_sums
    }

}
