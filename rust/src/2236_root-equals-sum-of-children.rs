/*
 * @lc app=leetcode id=2236 lang=rust
 *
 * [2236] Root Equals Sum of Children
 *
 * https://leetcode.com/problems/root-equals-sum-of-children/description/
 *
 * algorithms
 * Easy (90.00%)
 * Likes:    247
 * Dislikes: 427
 * Total Accepted:    31K
 * Total Submissions: 34.5K
 * Testcase Example:  '[10,4,6]'
 *
 * You are given the root of a binary tree that consists of exactly 3 nodes:
 * the root, its left child, and its right child.
 *
 * Return true if the value of the root is equal to the sum of the values of
 * its two children, or false otherwise.
 *
 *
 * Example 1:
 *
 *
 * Input: root = [10,4,6]
 * Output: true
 * Explanation: The values of the root, its left child, and its right child are
 * 10, 4, and 6, respectively.
 * 10 is equal to 4 + 6, so we return true.
 *
 *
 * Example 2:
 *
 *
 * Input: root = [5,3,1]
 * Output: false
 * Explanation: The values of the root, its left child, and its right child are
 * 5, 3, and 1, respectively.
 * 5 is not equal to 3 + 1, so we return false.
 *
 *
 *
 * Constraints:
 *
 *
 * The tree consists only of the root, its left child, and its right child.
 * -100 <= Node.val <= 100
 *
 *
 */
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// @lc code=start
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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let rootnode = root.as_ref().unwrap().borrow();
        let rightnode = rootnode.right.as_ref().unwrap().borrow();
        let leftnode = rootnode.left.as_ref().unwrap().borrow();
        rootnode.val == leftnode.val + rightnode.val
    }
}
// @lc code=end
