use core::task;

use crate::Solution;

use crate::ListNode;

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut head;

        let mut carry = 0;

        while (l1.is_some() || l2.is_some() || carry != 0) {
            let mut sum = carry;
            sum += l1.as_ref().map_or(0, |n| n.val);
            sum += l2.as_ref().map_or(0, |n| n.val);
            carry = sum / 10;
            sum %= 10;
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
            tail = &mut tail.as_mut().unwrap().next;
            l1 = l1.and_then(|n| n.next);
            l2 = l2.and_then(|n| n.next);
        }

        // because we made a dummy head that's just 0
        return head.unwrap().next;
    }
}
