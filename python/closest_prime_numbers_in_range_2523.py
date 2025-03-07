class Solution:
    def closestPrimes(self, left: int, right: int) -> list[int]:
        def sieve(n):
            marked = set()
            primes = []
            for i in range(2, n + 1):
                if i not in marked:
                    primes.append(i)
                    for multiple in range(i * i, n + 1, i):
                        marked.add(multiple)
            return primes

        if right < 2:
            return [-1, -1]

        prime_numbers = sieve(right)

        valid_primes = [p for p in prime_numbers if left <= p <= right]

        if len(valid_primes) < 2:
            return [-1, -1]

        min_diff = float('inf')
        closest_pair = [-1, -1]

        for i in range(1, len(valid_primes)):
            diff = valid_primes[i] - valid_primes[i - 1]
            if diff < min_diff:
                min_diff = diff
                closest_pair = [valid_primes[i - 1], valid_primes[i]]

        return closest_pair

