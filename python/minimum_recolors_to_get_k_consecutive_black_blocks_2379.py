class Solution:
    def minimumRecolors(self, blocks: str, k: int) -> int:
        #feels very simmilar to the minimums swaps for k consecutive buldings problem 


        black = 0
        best = 0
        cur = 0
        l = 0
        r = 0

        # two pointer sliding window to see the window with the best amount 
        # then subtract that best amount from the window 
        while r < len(blocks):
            if r - l >= k:
                if blocks[l] == 'B':
                    cur -=1
                l += 1
                continue
            if blocks[r] == 'B':
                cur += 1
            r+=1
            
            best = max(cur, best)
        return k - best

