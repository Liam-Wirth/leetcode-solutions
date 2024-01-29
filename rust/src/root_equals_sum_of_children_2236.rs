use crate::{TreeNode, Rc, RefCell};

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
use crate::Solution;
impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let rootnode = root.as_ref().unwrap().borrow();
        let rightnode = rootnode.right.as_ref().unwrap().borrow();
        let leftnode = rootnode.left.as_ref().unwrap().borrow();
        rootnode.val == leftnode.val + rightnode.val
    }
}
// @lc code=end
