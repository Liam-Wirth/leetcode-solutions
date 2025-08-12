#TODO: Revisit
class Solution:
    def largestIsland(self, grid: List[List[int]]) -> int:
        n = len(grid)
        UP, DOWN, LEFT, RIGHT = (0,1), (0,-1), (-1,0), (1,0)
        def dfs(x, y, idx):
            if x < 0 or x >= n or y < 0 or y >= n or grid[x][y] != 1:
                return 0
            grid[x][y] = idx
            return 1 + sum(dfs(x+dx, y+dy, idx) for dx, dy in [UP, DOWN, LEFT, RIGHT])

