use crate::Solution;
use std::collections::HashMap;
impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut num_to_freq: HashMap<i32,i32> = HashMap::new();
        for i in deck.iter(){
            *num_to_freq.entry(*i).or_insert(0)+=1;
        }
        let mut group_size: i32 = -1;
        for (key, value) in num_to_freq.iter() {
            if group_size == -1 {
                group_size = *value;
            } else {
                group_size = Self::gcd(group_size, *value);
            }
            
            }
            return group_size >=2;
    }

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b!= 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}
}