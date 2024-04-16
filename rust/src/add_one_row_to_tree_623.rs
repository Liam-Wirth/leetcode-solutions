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
use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        v: i32,
        d: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if d == 1 {
            Some(Rc::new(RefCell::new(TreeNode {
                val: v,
                left: root,
                right: None,
            })))
        } else {
            Self::dfs(&root, v, d - 2);
            root
        }
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, v: i32, d: i32) {
        if let Some(n) = node {
            if d > 0 {
                Self::dfs(&n.borrow().left, v, d - 1);
                Self::dfs(&n.borrow().right, v, d - 1);
            } else {
                let mut n = n.borrow_mut();
                n.left = Some(Rc::new(RefCell::new(TreeNode {
                    val: v,
                    left: n.left.take(),
                    right: None,
                })));
                n.right = Some(Rc::new(RefCell::new(TreeNode {
                    val: v,
                    left: None,
                    right: n.right.take(),
                })));
            }
        }
    }
}
