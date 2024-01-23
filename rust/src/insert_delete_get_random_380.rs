use std::{collections::HashMap, vec};
use rand::prelude::*;
struct RandomizedSet {
    list: Vec<i32>,
    set: HashMap<i32, usize>, 
}

#[cfg(not(test))]
/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        Self { list: Vec::new(), set: HashMap::new() }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if !self.set.contains_key(&val) {
            self.set.insert(val, self.list.len());
            self.list.push(val);
            true
        } else {false}
    }
    
    fn remove(&mut self, val: i32) -> bool {
        if let Some(index) = self.set.remove(&val) {
            if let Some(&val2) = self.list.last() {
                self.list.swap_remove(index);
                if val != val2 {
                    *self.set.get_mut(&val2).unwrap() = index;
                }
            }
            true
        } else { false } 
    }
    
    fn get_random(&mut self) -> i32 {
        *self.list.choose(&mut rand::thread_rng()).unwrap()
    }
}
