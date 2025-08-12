// TODO: Could be worth revisiting, as my original implementation TLEs due to constant sorting
// being less efficient then using a btree/ordered set
//
// first attempt solving this problem took 30 minutes
use crate::Solution;
use std::collections::HashMap;
use std::collections::HashSet;
struct NumberContainers {
    // (index, number at the index)
    nums: HashMap<i32, i32>,
    // number, where it is in the list
    locations: HashMap<i32, Vec<i32>>, // Might be better to replace this with a hashSet
                                       //locations: HashMap<i32, HashSet<i32>>, // thought so, but then I realized we need the list to
                                       //be sorted
}

impl NumberContainers {
    fn new() -> Self {
        Self {
            nums: HashMap::new(),
            locations: HashMap::new(),
        }
    }

    // Mutates the list, replaces value at index with number, needs to update locations to remove
    // the location of the old number, and update the locations list of the new number as well
    fn change(&mut self, index: i32, number: i32) {
        if let Some(&old_number) = self.nums.get(&index) {
            if let Some(locs) = self.locations.get_mut(&old_number) {
                locs.retain(|&x| x != index); // Remove the index from the old number's location list
                if locs.is_empty() {
                    self.locations.remove(&old_number); // Remove the entry if the list is empty
                }
            }
        }
        self.nums.insert(index, number); // Insert or update the number at the index
        self.locations
            .entry(number)
            .or_insert_with(Vec::new)
            .push(index); // Add the index to the new number's location list
        self.locations.get_mut(&number).unwrap().sort(); // Ensure the location list is sorted
    }

    // Binary Search
    fn find(&self, number: i32) -> i32 {
        if let Some(ret) = self.locations.get(&number) {
            if !ret.is_empty() {
                return ret[0];
            }
        }
        -1
    }
}

use std::collections::{BTreeSet, HashMap};

struct NumberContainers2 {
    nums: HashMap<i32, i32>,
    locations: HashMap<i32, BTreeSet<i32>>,
}

impl NumberContainers2 {
    fn new() -> Self {
        Self {
            nums: HashMap::new(),
            locations: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        if let Some(&old_number) = self.nums.get(&index) {
            if let Some(locs) = self.locations.get_mut(&old_number) {
                locs.remove(&index); // Remove the index from the old number's location set
                if locs.is_empty() {
                    self.locations.remove(&old_number); // Remove the entry if the set is empty
                }
            }
        }
        self.nums.insert(index, number); // Insert or update the number at the index
        self.locations
            .entry(number)
            .or_insert_with(BTreeSet::new)
            .insert(index); // Add the index to the new number's location set
    }

    fn find(&self, number: i32) -> i32 {
        if let Some(locs) = self.locations.get(&number) {
            if let Some(&smallest_index) = locs.iter().next() {
                return smallest_index; // Return the smallest index
            }
        }
        -1 // Return -1 if the number is not found
    }
}
/*
 * Your NumberContainers object will be instantiated and called as such:
 * let obj = NumberContainers::new();
 * obj.change(index, number);
 * let ret_2: i32 = obj.find(number);
 */

