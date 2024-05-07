use crate::ListNode;
use crate::Solution;
// Definition for singly-linked list.
impl Solution {
    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut prev = head.as_mut()?;
        while let Some(digit) = prev.next.as_mut() {
            prev.val += (digit.val * 2) / 10;
            digit.val = (digit.val * 2) % 10;
            prev = digit;
        }
        if head.as_ref().is_some_and(|v| v.val == 0) {
            head = head?.next.take();
        }
        head
    }
}
