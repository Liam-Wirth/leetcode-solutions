/*You are given a tree (i.e. a connected, undirected graph that has no cycles) consisting of n nodes numbered from 0 to n - 1 and exactly n - 1 edges. The root of the tree is the node 0, and each node of the tree has a label which is a lower-case character given in the string labels (i.e. The node with the number i has the label labels[i]).

The edges array is given on the form edges[i] = [ai, bi], which means there is an edge between nodes ai and bi in the tree.

Return an array of size n where ans[i] is the number of nodes in the subtree of the ith node which have the same label as node i.

A subtree of a tree T is the tree consisting of a node in T and all of its descendant nodes.
 *
 * */
pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
    let mut adj = vec![Vec::new(); n as usize];
    for e in edges.iter() {
        adj[e[0] as usize].push(e[1] as usize);
        adj[e[1] as usize].push(e[0] as usize);
    }
    let mut ans = vec![0; n as usize];
    dfs(n as usize, 0, &mut ans, &adj, labels.as_bytes());
    ans
}
pub fn dfs(
    prev: usize,
    curr: usize,
    ans: &mut Vec<i32>,
    adj: &Vec<Vec<usize>>,
    bytes: &[u8],
) -> [i32; 26] {
    let mut count = [0; 26];
    for &i in adj[curr].iter() {
        if i != prev {
            let count2 = dfs(curr, i, ans, adj, bytes);
            for j in 0..26 {
                count[j] += count2[j];
            }
        }
    }
    count
}

pub fn main() {}
