use crate::Solution;
use std::collections::HashMap;
use rand::prelude::*;

impl Solution{
pub struct RandomizedSet {
    list: Vec<i32>,
    set: HashMap<i32, usize>, 
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
pub impl RandomizedSet {

    pub fn new() -> Self {
        Self {
            list: Vec::new(i32),
            set: HashMap::new(),
        }
    }    
    pub fn insert(&self, val: i32) -> bool {
       if !self.set.contains(val) {
           self.set.insert()
       }  
    }
    
    pub fn remove(&self, val: i32) -> bool {
        
    }
    
    pub fn get_random(&self) -> i32 {
        
    }
}
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
