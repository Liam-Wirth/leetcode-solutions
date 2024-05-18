// TODO: Finish this ho, and revisit it later
//
use crate::Solution;
use std::collections::binary_heap;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Eq, PartialEq)]
struct Cell {
    x: usize,
    y: usize,
    value: i32,
}

// Implement PartialOrd and Ord for Cell to make it usable in a BinaryHeap
// Reverse is used to turn the max-heap into a min-heap
impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Reverse(self.value).partial_cmp(&Reverse(other.value))
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
const DIRS: [(i32, i32,); 4]=[(0,1), (0,-1), (1, 0), (-1, 0)];
use std::collections::VecDeque;
impl Solution {
    pub fn maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
        //ensuring the least proximity to any cell containing a thief
        //proximity is determined by manhattan distance
        let n: usize = grid.len();
        let mut heap: BinaryHeap<Cell> = binary_heap::BinaryHeap::new();
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visited = HashSet::new();
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    q.push_back((i,j));
                    grid[i][j] = 0; //Marking thieves with 0
                }else {
                    grid[i][j] = -1; //Marking safe cells with -1
                }
            }
        }


        while q.len() > 0 {
            let (i,j) = q.pop_front().unwrap();
            if !visited.insert((i,j)){
                q.pop_front();
                continue;
            }
            for (dx, dy) in DIRS.iter() {
                let x = i as i32 + dx;
                let y = j as i32 +dy;
                if 0 <= x && x < grid.len() as i32 && 0 <= y && y < grid.len() as i32 {
                    let x = x as usize;
                    let y = y as usize;
                if grid[x][y] == -1 {
                    grid[x][y] = grid[i][j] + 1;
                }
                q.push_back((x as usize,y as usize));
            }
        }
    }
    // now we do dijkstra :3
    grid[0][0]=-1;
    for i in 0..n {
        for j in 0..n {
            heap.push(Cell{x:i, y:j, value: grid[i][j]});
        }
    }

    while heap.len() > 0 {
        let cur = heap.pop().unwrap();
        let safety = cur.value;
        let x = cur.x;
        let y = cur.y;

        if x == (n-1)&& (n-1) == y {
            return safety;
        }

        for (dx,dy) in DIRS.iter() {
            let mut x = x as i32 + dx;
            let mut y = y as i32 + dy;
            if 0 <= x && x < grid.len() as i32 
            && 0 <= y && y < grid.len() as i32
            && grid[x as usize][y as usize] != -1 {
                let x = x as usize;
                let y = y as usize;
                let new_safety = safety.max(grid[x][y]);
                heap.push(Cell{x:x, y:y, value: new_safety})
            }
            
        }
    }
    0
    }
}
