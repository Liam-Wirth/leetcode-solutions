class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        # no duplicates, must contain 1-9, doesn't need all of them
        # the board is 9x9, so to get cols we can do this
        # also might have to do some weird ass sliding window shit for 3x3?
        for row in board:
            seen = set()
            for n in row:
                if n != '.' and n in seen:
                    return False #early exit
                seen.add(n)

        #lets do it for rows now,
        # taking notes cause this is tricky for me, using python unpacking operator to turn the 2d list into like an iterator over its rows
        # calling *board is like doing board[0]..board[n]
        for col in zip(*board):
            seen = set()
            for n in col:
                if n != "." and n in seen:
                    return False
                seen.add(n)


        #some fucky shit for the 3x3s
        for i in range(0, 9, 3):  # Increment i by 3 in the outer loop
            for j in range(0, 9, 3):  # Increment j by 3 in the outer loop
                seen = set()

                for row in range(i, i + 3):  # i is already the correct starting row
                    for col in range(j, j + 3): # j is already the correct starting col
                        n = board[row][col]
                        if n != '.' and n in seen:
                            return False
                        seen.add(n)
        
        return True
        
