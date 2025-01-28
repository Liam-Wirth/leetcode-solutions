use std::collections::HashSet;

use crate::Solution;
// need to do a topo-sort
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prereq = {
            let mut adj_list = vec![vec![]; num_courses as usize];
            for edge in prerequisites.iter() {
                adj_list[edge[0] as usize].push(edge[1] as usize);
            }
            adj_list
        };
        println!("{:?}", prereq);

        // course has 3 possible states
        // visited, in output
        // visiting, not added but part of cycle
        // unvisited not added to output or cycle

        let mut visit = HashSet::new();
        let mut cycle = HashSet::new();

    }
    fn find_order_dfs() {

    }

}

