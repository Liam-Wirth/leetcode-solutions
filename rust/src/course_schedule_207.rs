use crate::Solution;
use std::collections::HashSet;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut adj_list = {
            let mut adj_list = vec![vec![]; num_courses as usize];
            for edge in prerequisites.iter() {
                adj_list[edge[0] as usize].push(edge[1] as usize);
            }
            adj_list
        };

        let mut visited: HashSet<usize> = HashSet::new();

        for course in 0..num_courses as usize {
            if !visited.contains(&course)
                && !Solution::can_finish_dfs(&mut adj_list, &mut visited, course)
            {
                return false;
            }
        }
        true
    }

    fn can_finish_dfs(
        adj_list: &mut Vec<Vec<usize>>,
        visited: &mut HashSet<usize>,
        course: usize,
    ) -> bool {
        if visited.contains(&course) {
            // found a loop thus impossible
            return false;
        }

        if adj_list[course].is_empty() { 
            return true;
        }

        visited.insert(course);

        // Actual DFS
        for crs in adj_list[course].clone() {
            if !Solution::can_finish_dfs(adj_list, visited, crs) {
                return false;
            }
        }
        visited.remove(&course);
        adj_list[course] = [].to_vec();
        true
    }
}

