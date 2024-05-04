use crate::Solution;
impl Solution {
    pub fn num_rescue_boats(mut ppl: Vec<i32>, limit: i32) -> i32 {
        //let mut out: i32 = people.iter().filter(|&i| i >= &limit).count().try_into().unwrap();
        //any people at the weight limit of the boat will take one boat
        //let mut ppl: Vec<i32> = people.iter().filter(|&i| i < &limit).cloned().collect(); //filtering out said people
        let mut out = 0;
        ppl.sort();

        let mut front = 0;
        let mut back = ppl.len() - 1;
        while back >= front {
            out += 1;
            let i = ppl[front];
            let j = ppl[back];

            if i + j <= limit {
                front += 1;
            }
            if back == 0 {
                break;
            }
            back -= 1;
        }

        out
    }
}
