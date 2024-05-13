class Solution:
    def largestLocal(self, grid):
        n = len(grid)

        for i in range(1, n - 1):
            for j in range(1, n - 1):
                temp = 0

                for k in range(i - 1, i + 2):
                    for l in range(j - 1, j + 2):
                        temp = max(temp, grid[k][l])

                grid[i - 1][j - 1] = temp

        n = len(grid)
        grid = [row[:n-2] for row in grid[:n-2]]

        return grid
