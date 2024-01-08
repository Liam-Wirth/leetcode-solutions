struct Solution {}
impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        //big learning for me, the Scan function in rust allows me to maintain a certain
        //value/state while iterating over the array, basically making it so I can keep track of
        //the sum through each iteration.
        let prefix_sum = nums
            .iter()
           .scan(0, |sum, i| {
                *sum += i;
                Some(*sum)
            })
            .collect::<Vec<_>>();
        let mut count = vec![0; k as usize];
        count[0] = 1;
        let mut answer = 0;
        for mut i in prefix_sum {
            if i < 0 {
                i = (i % k + k) % k;
            }
            count[(i % k) as usize] += 1;
        }
        for i in count {
            answer += i * (i - 1) / 2;
        }
        answer
    }
}

fn main() {}
