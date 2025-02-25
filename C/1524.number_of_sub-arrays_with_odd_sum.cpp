#define MOD 1000000007

// Need to get the prefix sum, but for each index as if it's zero, so I might
// need to do a nested for loop, for example if I have arr [1, 2, 3]
/*

prefix sum starting from i = 0:
[1, 3, 6]

but then, I'd want the prefix sums starting from i++

[2, 5]

and again i++ (i = 2, which is also n (n = len))

[3]


so the answer of all sums would be:
[1, 2, 3, 3, 5, 6]
and the total number of ODD sums is 4




*/

class Solution {
public:
  int numOfSubarrays(vector<int> &arr) {

    int n = arr.size();
    int answer = 0;    // This will store the number of subarrays with odd sums
    int prefixSum = 0; // To store the current prefix sum
    int oddCount = 0,
        evenCount = 1; // EvenCount starts at 1 because the sum of an empty
                       // subarray is considered even

    // Traverse the array and calculate prefix sums
    for (int i = 0; i < n; ++i) {
      prefixSum += arr[i]; // Add the current element to the running prefix sum

      // If the current prefix sum is even, the subarray count depends on
      // the previous odd subarrays
      if (prefixSum % 2 == 0) {
        answer = (answer + oddCount) %
                 MOD; // Add the count of odd prefix sums to the answer
        evenCount++;  // Increment the count of even prefix sums
      }
      // If the current prefix sum is odd, the subarray count depends on
      // the previous even subarrays
      else {
        answer = (answer + evenCount) %
                 MOD; // Add the count of even prefix sums to the answer
        oddCount++;   // Increment the count of odd prefix sums
      }
    }

    return answer;
  }
  int oldNumSubarrs(vector<int> &arr) {

    int answer = 0; // Initialize the answer to 0 as we're going to compute it
    int n = arr.size();

    vector<vector<int>> sums(
        n, vector<int>(
               n, 0)); // Initialize the 2D vector 'sums' to store prefix sums

    // Compute all prefix sums
    for (int i = 0; i < n; ++i) {
      // Initialize the first element in each row with the corresponding
      // arr[i]
      sums[i][0] = arr[i];

      // Calculate prefix sums for each subarray starting from index i
      for (int j = 1; j < n - i; ++j) {
        sums[i][j] = sums[i][j - 1] +
                     arr[i + j]; // Add the current element to the previous sum
      }
    }

    // Count the number of subarrays with odd sums
    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < n - i; ++j) {
        if (sums[i][j] % 2 != 0) { // If the sum is odd
          answer =
              (answer + 1) % MOD; // Increment the answer and take modulo MOD
        }
      }
    }

    return answer;
  }
};
