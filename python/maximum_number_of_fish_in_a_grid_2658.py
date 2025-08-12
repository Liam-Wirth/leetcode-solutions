class Solution:
    def findMaxFish(self, grid: list[list[int]]) -> int:
        visited: set[tuple[int, int]] = set()
        max_fish: int = 0

        def dfs(x: int, y: int, fish: int) -> int:
            """
            base cases:
            x<0
            y<0
            x>=len(grid)
            y>=len(grid[0])
            (x, y) in visited
            """
            # think about base cases,  x<= 0, y <=0 or x>= len(list)

            if x < 0 or y < 0 or x >= len(grid) or y >= len(grid[0]) or (x, y) in visited:
                return 0
            visited.add((x, y))
            fish += grid[x][y]
            # now determine which directions we can go:
            # (check up down left right, can go to nonzero cells)
            # current cell is (x, y)
            # up: (x-1, y)
            # down: (x+1, y)
            # left: (x, y-1)
            # right: (x, y+1)
            if x - 1 >= 0 and grid[x - 1][y] > 0:
                fish = max(fish, dfs(x - 1, y, fish))
            if x + 1 < len(grid) and grid[x + 1][y] > 0:
                fish = max(fish, dfs(x + 1, y, fish))
            if y - 1 >= 0 and grid[x][y - 1] > 0:
                fish = max(fish, dfs(x, y - 1, fish))
            if y + 1 < len(grid[0]) and grid[x][y + 1] > 0:
                fish = max(fish, dfs(x, y + 1, fish))
            return fish

        for i in range(len(grid)):
            for j in range(len(grid[0])):
                if grid[i][j] > 0:
                    max_fish = max(max_fish, dfs(i, j, 0))
                    visited.clear()
        return max_fish

