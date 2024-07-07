class Solution:
    def mergeNodes(self, head: Optional[ListNode]) -> Optional[ListNode]:
        dummy = ListNode(0)
        cur_out = dummy
        cur = head.next
        running_val = 0

        while cur is not None:
            if cur.val != 0:
                running_val += cur.val
            else:
                if running_val != 0:
                    cur_out.next = ListNode(running_val)
                    cur_out = cur_out.next
                    running_val = 0
            cur = cur.next

        return dummy.next
