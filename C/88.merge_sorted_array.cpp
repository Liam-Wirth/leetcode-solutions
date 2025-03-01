#include <deque>
#include <vector>
using namespace std;
class Solution {
public:
  void merge(vector<int> &nums1, int m, vector<int> &nums2, int n) {
    vector<int> out = vector<int>(m + n); // declare a new vector for the result

    int idx1 = m - 1;
    int idx2 = n - 1;
    int tot = m + n - 1;

    while (idx2 >= 0) {
      if (idx1 >= 0 && nums1[idx1] > nums2[idx2]) {
        nums1[tot--] = nums1[idx1--];
      } else {
        nums1[tot--] = nums2[idx2--];
      }
    }
  }
};
