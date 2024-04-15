use crate::Solution;
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(root, 0)
    }
   fn dfs(root: Option<Rc<RefCell<TreeNode>>>, prev_sum: i32) -> i32 {
    if let Some(node) = root {
        let node_borrow = node.borrow();

        // Calculate the current sum
        let current_sum = prev_sum * 10 + node_borrow.val;

        // If it's a leaf node, return the sum
        if node_borrow.left.is_none() && node_borrow.right.is_none() {
            return current_sum;
        }

        // Otherwise, continue DFS
        let left_sum = Self::dfs(node_borrow.left.clone(), current_sum);
        let right_sum =Self::dfs(node_borrow.right.clone(), current_sum);

        // Return the sum of left and right branches
        return left_sum + right_sum;
    }

    0
}

}
