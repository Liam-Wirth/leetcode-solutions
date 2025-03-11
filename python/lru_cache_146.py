from typing import Optional

class ListNode:
    def __init__(self, key, val):
        self.prev: Optional["ListNode"] = None
        self.next: Optional["ListNode"] = None
        self.key = key
        self.val = val


class LRUCache:
    def __init__(self, capacity: int):
        self.cap = capacity
        self.hash = {}
        self.head = None
        self.tail = None

    def remove(self, node: ListNode):
        if node.prev:
            node.prev.next = node.next
        else:  # node is head
            self.head = node.next

        if node.next:
            node.next.prev = node.prev
        else:  # node is tail
            self.tail = node.prev

        node.prev = None
        node.next = None

    def insert_head(self, node: ListNode):
        node.next = self.head
        node.prev = None
        if self.head:
            self.head.prev = node
        self.head = node

        if self.tail is None:
            self.tail = node

    def get(self, key: int) -> int:
        if key not in self.hash:
            return -1
        node = self.hash[key]
        self.remove(node)
        self.insert_head(node)
        return node.val

    def put(self, key: int, value: int) -> None:
        if key in self.hash:  # cache hit
            node = self.hash[key]
            node.val = value
            self.remove(node)
            self.insert_head(node)
        else:
            if len(self.hash) >= self.cap:
                if self.tail:
                    lru = self.tail
                    self.remove(lru)
                    del self.hash[lru.key]
            new_node = ListNode(key, value)
            self.insert_head(new_node)
            self.hash[key] = new_node
