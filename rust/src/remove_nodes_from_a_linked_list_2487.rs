use crate::Solution;
use crate::ListNode;
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn remove_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut hrev = None;
        let mut last = 0;

        // Reverse the list and move to hrev.
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = hrev;
            hrev = Some(node);
        }
        while let Some(mut node) = hrev {
            hrev = node.next.take();
            if node.val >= last {
                node.next = head;
                last = node.val;
                head = Some(node);
            }
        }
        head
    }
}
