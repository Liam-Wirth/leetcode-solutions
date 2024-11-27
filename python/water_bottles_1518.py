class Solution:
    def numWaterBottles(self, numBottles, numExchange):
        #    total_drunk = 0
        #    empty_bottles = 0
        #
        #    while numBottles > 0:
        #        total_drunk += numBottles
        #        empty_bottles += numBottles
        #        numBottles = empty_bottles // numExchange
        #        empty_bottles = empty_bottles % numExchange
        #    return total_drunk
        #
        if numExchange > numBottles:
            return numBottles
        else:
            return numBottles + (numBottles - 1) // (numExchange - 1)
