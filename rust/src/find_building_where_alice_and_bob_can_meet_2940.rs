use crate::Solution;

// TODO: Revisit this, I'm too dumb. need to study more monotonic stack stuff
impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = heights.len();
        let mut next_higher = vec![vec![]; n];
        let mut stack = Vec::new();
        
        // For each position, precompute all reachable positions to its right
        // that are higher than it
        for i in (0..n).rev() {
            while !stack.is_empty() && heights[*stack.last().unwrap()] <= heights[i] {
                stack.pop();
            }
            if !stack.is_empty() {
                next_higher[i].push(*stack.last().unwrap());
                // Add all positions reachable from the next higher building
                let mut curr: usize = *stack.last().unwrap();
                let var_name: Vec<usize> = next_higher[curr].clone();
                var_name.iter().for_each(|&next| {
                    if heights[next] > heights[i] {
                        next_higher[i].push(next);
                    }
                });
            }
            stack.push(i);
        }

        queries.iter().map(|q| {
            let mut a = q[0] as usize;
            let mut b = q[1] as usize;
            
            // If same building, return it
            if a == b {
                return a as i32;
            }
            
            // Ensure a is the smaller index
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            
            // If b is higher than a, return b
            if heights[b] > heights[a] {
                return b as i32;
            }
            
            // Find the first building reachable by both
            let target_height = heights[a].max(heights[b]);
            
            // Check reachable buildings from both positions
            let reachable_a = &next_higher[a];
            let reachable_b = &next_higher[b];
            
            let mut i = 0;
            let mut j = 0;
            
            while i < reachable_a.len() && j < reachable_b.len() {
                if reachable_a[i] == reachable_b[j] {
                    return reachable_a[i] as i32;
                } else if reachable_a[i] < reachable_b[j] {
                    i += 1;
                } else {
                    j += 1;
                }
            }
            
            -1
        }).collect()
    }
}
