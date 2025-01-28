use std::collections::HashSet;
use std::collections::VecDeque;
use crate::Solution;

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        //wait lets get our starting state first actually:
        
        let mut state = Lock::new("0000");
        let target = Lock::new(target.as_str());
        let mut ignore: HashSet<Lock> = deadends.iter().map(|dead| Lock::new(dead)).collect();
        let mut visited: HashSet<Lock> = HashSet::new();
        if ignore.contains(&target) || ignore.contains(&state) {
            return -1;
        }

        if target == state {
            return 0;
        }

        let mut q: VecDeque<(Lock, i32)> = VecDeque::new();
        q.push_back((state, 0));
        while !q.is_empty() {
            let cur = q.pop_front().unwrap();
            if visited.contains(&cur.0) {
                continue;
            }
            visited.insert(cur.0.clone());
            
            if cur.0 == target {
                //for elem in visited{
                //    println!("{:?}", elem);
                //}
                
                return cur.1;

            }

            for nxt in cur.0.next() {
                if !visited.contains(&nxt) && !ignore.contains(&nxt) {
                    q.push_back((nxt, cur.1 + 1));
                }
                if ignore.contains(&nxt) {

                }
            }
        }
        
        //Constraints say that starting state could be a deadend
        -1
    }
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Lock {
    //let me cook
    state: [i8; 4],
}

impl Lock {
    fn new(state: &str) -> Self {
        //RAHHHHHH I LOVE ITERATORS RAHHHHHHHHHHH
        //for i in state.parse().unwrap().into_iter() {
        //    lock.state[i] = i;
        //} nvm
        let state = state.as_bytes();
        return Lock {
            state: [
                (state[0] - '0' as u8) as i8,
                (state[1] - '0' as u8) as i8, 
                (state[2] - '0' as u8) as i8, 
                (state[3] - '0' as u8) as i8, 
            ],
        };
    }
    fn default() -> Self {
        Lock { state: [0; 4] }
    }
    /// Returns the next possible states of the lock
    fn next(&self) -> Vec<Lock> {
        let mut out = Vec::new();

        for i in 0..4 {
            let mut next = self.clone();
            next.state[i] = (next.state[i] + 1) % 10;

            out.push(next);

            //consider the inverse (going down by 1)
            let mut next = self.clone();

            if next.state[i] - 1 < 0 {
                next.state[i] = 9;
            } else {
                next.state[i] -= 1;
            }
            //we get all instances now forward and back spinning of the wheel
            out.push(next);
        }

        out
    }
}

