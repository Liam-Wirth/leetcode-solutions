#include <cstdint>
#include <string>
#include <unordered_set>
#include <vector>
// NOTE: xor?
using namespace std;
class Solution {
public:
  // given an array of binary strings each of length n, return a binary string
  // of length n that does not appear in nums
  string findDifferentBinaryString(vector<string> &nums) {
    // gonna think about this numerically, so I'm just gonna first observe the
    // length N
    auto N = nums.size();

    std::unordered_set<string> s;
    for (string st : nums) {
      s.insert(st);
    }
    // binary and check the set till we see it
    // need to find some combination of 0s and 1s that is not in the set
    // brute force until we find a string that is not in the set
    for (int i = 0; i < (1 << N); ++i) {
      string candidate(N, '0');
      for (int j = 0; j < N; ++j) {
        if (i & (1 << j)) {
          candidate[N - j - 1] = '1';
        }
      }
      if (s.find(candidate) == s.end()) {
        return candidate;
      }
    }
    return ""; // This line should never be reached
  }
};

// DIAGONALIZATION!!! YES YES!!!
class Solution {
public:
  string findDifferentBinaryString(vector<string> &nums) {
    int n = nums.size();
    string res(n, '0');
    // iter from 0 to n-1:
    // if the ith character of ith string is 0 append 1 to res, else append 0
    // gaurunteed to be unique
    for (int i = 0; i < n; ++i) {
      res[i] = nums[i][i] == '0' ? '1' : '0';
    }
    return res;
  }
}
