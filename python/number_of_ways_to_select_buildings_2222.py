class Solution:
    def numberOfWays(self, s: str) -> int:

        # iterate over the list from a pointer of l to r/ dynamic window size
        # basically untilt we've selected every single possible combination of 3 values, wait nao

        #wait, no, this is silly, it's just 3, I can write down all combinations of 3 in binary (3^2) and then be able to just find ones that can/cannot make a valid choice
        #010 101 
        # Only two "valid" selections, so we just match against those

        # prefix sums of numbers up to that point
        zeroes = [0] * len(s)
        ones = [0]  * len(s)

        z = 0
        o = 0
        for i in range(len(s)):
            if s[i] == '0':
                z +=1
            else:
                o += 1
            
            ones[i] = o
            zeroes[i] = z


        out = 0
        n = len(s)

        for j in range(1, n - 1): # j must be at least index 1 and at most n-2 to have i<j<k
            if s[j] == '1': # Case 1: "010" pattern
                zeroes_before_j = zeroes[j-1] if j > 0 else 0
                zeroes_after_j = zeroes[n-1] - zeroes[j]
                out += zeroes_before_j * zeroes_after_j
            elif s[j] == '0': # Case 2: "101" pattern
                ones_before_j = ones[j-1] if j > 0 else 0
                ones_after_j = ones[n-1] - ones[j]
                out += ones_before_j * ones_after_j

        return out
                

