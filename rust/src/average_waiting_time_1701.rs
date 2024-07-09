use crate::Solution;
impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut curr_time: i64 = customers[0][0] as i64;
        let mut wait_sum: i64 = 0;
        let cust_count = customers.len() as f64;
        for i in customers {
            if (i[0] as i64) > curr_time {
                curr_time = (i[0] as i64);
            }
            curr_time += (i[1] as i64);
            wait_sum += curr_time - (i[0] as i64);
        }
        (wait_sum as f64) / cust_count
    }
}

