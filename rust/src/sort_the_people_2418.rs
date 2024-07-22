use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut hm = HashMap::new();
        for i in 0..names.len() {
            hm.entry(heights[i].clone()).or_insert(names[i].clone()); // Store heights as keys and names as values
        }

        // Collect the HashMap into a vector of tuples and sort by heights
        let mut people: Vec<(i32, String)> = hm.into_iter().collect();
        people.sort_by(|a, b| b.0.cmp(&a.0)); // Sort by height in descending order

        // Extract the names from the sorted vector
        let sorted_names: Vec<String> = people.into_iter()
            .map(|(_, name)| name) // Using map to effectively ignore the height value, and only
                                   // grab the name value (hence the use of the underscore)
            .collect();

        sorted_names
    }
}

