from typing import Optional
# Definition for singly-linked list.
 class ListNode:
     def __init__(self, val=0, next=None):
         self.val = val
         self.next = next
class Solution:

    def deleteDuplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:
        seen = set()

        walk = head

        while walk:
            if walk.val in seen:
                if walk.next:
                    walk.next = walk.next.next
            else:
                seen.add(walk.val)
            walk = walk.next
        return head


    def deleteDuplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:
        walk = head

        while walk:
            if walk.next and walk.val == walk.next.val:
                walk.next = walk.next.next
            else:
                walk = walk.next
        return head
