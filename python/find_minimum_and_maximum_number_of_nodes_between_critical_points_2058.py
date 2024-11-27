# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def nodesBetweenCriticalPoints(self, head: Optional[ListNode]) -> List[int]:
        if not head or not head.next or not head.next.next:
            return [-1, -1]

        crits = []
        prev = head
        curr = head.next
        pos = 1

        while curr.next:
            minima = curr.val < prev.val and curr.val < curr.next.val
            maxima = curr.val > prev.val and curr.val > curr.next.val

            if minima or maxima:
                crits.append(pos)
            prev = curr
            curr = curr.next
            pos += 1

        if len(crits) < 2:
            return [-1, -1]

        min_dist = float("inf")
        max_dist = crits[-1] - crits[0]

        for i in range(1, len(crits)):
            min_dist = min(min_dist, crits[i] - crits[i - 1])

        return [min_dist, max_dist]
