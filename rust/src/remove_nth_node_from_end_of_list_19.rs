use crate::{Solution, ListNode};
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut r = ListNode { val: 0, next: head };
        let mut r = Box::new(r);
        let mut b = r.clone();
        let mut a = r.as_mut();
        let mut i = 0;

        while b.next.is_some() {
            i += 1;
            if i > n {
                a = a.next.as_mut().unwrap()
            }
            b = b.next.unwrap();
        }
        let n = a.next.as_mut().unwrap().next.take();
        a.next = n;
        return r.next;
    }
}
