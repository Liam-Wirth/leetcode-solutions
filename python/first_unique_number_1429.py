class FirstUnique:

    def __init__(self, nums: List[int]):
        self.counts = {}
        self.q = deque()
        for num in nums:
            self.add(num)
        

    def showFirstUnique(self) -> int:
        while self.q and self.counts[self.q[0]] > 1:
            self.q.popleft()
        if not self.q:
            return -1
        return self.q[0]
        

    def add(self, value: int) -> None:
        self.counts[value] = self.counts.get(value, 0) + 1
        if self.counts[value] == 1:
            self.q.append(value)
        


# Your FirstUnique object will be instantiated and called as such:
# obj = FirstUnique(nums)
# param_1 = obj.showFirstUnique()
# obj.add(value)
