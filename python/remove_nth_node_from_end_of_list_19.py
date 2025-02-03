# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        # iterate through the list until you find the nth node from the end, then remove it.
        dummy = ListNode(0, head)
        slow = dummy
        fast = dummy
        steps = 0
        while fast.next:
            steps += 1
            if steps > n:
                slow = slow.next
            fast = fast.next
        remove = slow.next
        slow.next = remove.next

        return dummy.next

