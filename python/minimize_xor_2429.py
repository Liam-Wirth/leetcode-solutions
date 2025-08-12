class Solution:
    def minimizeXor(self, num1: int, num2: int) -> int:
        # Count target number of set bits in num2
        target_set_bits = bin(num2).count("1")
        
        # Initialize result and count of set bits in the result
        result = 0
        set_bits = 0

        # Iterate over bits from most significant (31) to least significant (0)
        for i in range(31, -1, -1):
            if (num1 & (1 << i)) != 0:  # If the current bit in num1 is set
                if set_bits < target_set_bits:  # Only set it in result if we still need set bits
                    result |= (1 << i)
                    set_bits += 1

        # If we still need more set bits after processing num1's bits
        for i in range(32):
            if set_bits < target_set_bits and (result & (1 << i)) == 0:
                result |= (1 << i)  # Set the least significant unset bit
                set_bits += 1

        return result

