class Solution:
    def queryResults(self, limit: int, queries: List[List[int]]) -> List[int]:
        res = []

        colors = {}
        balls = {}

        for i in queries:
            ball = i[0]
            col = i[1]

            old = balls.get(ball)
            if old is not None and old in colors:
                colors[old].remove(ball)
                if not colors[old]:
                    del colors[old]

            if colors.get(col):
                colors[col].append(ball)
            else:
                colors[col] = [ball]

            balls[ball] = col

            res.append(len(colors))
        return res
        

