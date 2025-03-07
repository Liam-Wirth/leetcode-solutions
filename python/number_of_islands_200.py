class Solution:
    def numIslands(self, grid: List[List[str]]) -> int:
        m = len(grid)
        n = len(grid[0])


        visited = [[False for _ in range(n)] for _ in range(m)]
        
        def valid(r, c):
            return 0 <= r < m and 0 <= c < n
        dirs = [(1,0),(0,1),(0,-1),(-1,0)]
        def bfs(r, c):
            q = deque([(r,c)])
            visited[r][c] = True
            while q:
                row, col = q.popleft()

                for d in dirs: # explore the neighbors columbus style
                    i = d[0] + row
                    j = d[1] + col
                    if valid(i,j):
                        #we can check this, so, let's add it to the q IF it's a 1, we need to see how far out this "island" goes
                        if grid[i][j] == '1' and not visited[i][j]:
                            q.append((i,j))
                            visited[i][j] = True

        out = 0
        for i in range(0,m):
            for j in range(0, n):
                if not visited[i][j]:
                    if grid[i][j] == '1':
                        bfs(i,j)
                        out +=1
                else:
                    continue
        
        return out

