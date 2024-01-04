use std::collections::HashSet;
//TODO: Resolve this one, I'm not really happy with the solution
pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
    let n = s.len();
    let s: Vec<char> = s.chars().collect();
    let mut ans = 0;
    for i in 0..n {
        let mut set = HashSet::new();
        set.insert(s[i]);
        ans = std::cmp::max(ans, dfs(i, &s, &mut set));
    }
    ans
}
fn dfs(i: usize, s: &Vec<char>, set: &mut HashSet<char>) -> i32 {
    //variable to store the maximum length we'll find
    let mut ans = 0;
    for j in i..s.len() {
        if !set.contains(&s[j]) {
            set.insert(s[j]);
            ans = std::cmp::max(ans, 1 + dfs(j + 1, s, set));
            set.remove(&s[j]);
        }
    }
    ans
}
fn main() {}
