#include <stdlib.h>
#include <vector>


using namespace std;


class Solution {
public:
    vector<int> productExceptSelf(vector<int>& nums) {
        int n = nums.size();
        vector<int> result(n);
        vector<int> prefix(n);
        vector<int> suffix(n); 


        prefix[0] = 1;
        suffix[n - 1] = 1;

        // compute the prefix sum array

        for (int i = 1; i < n; i++) {
            prefix[i] = nums[i-1] * prefix[i-1];
        }
        // suffix array
        for (int i = n -2; i >= 0; i--) {
            suffix[i] = nums[i+1] * suffix[i+1];
        }

        // get the answer

        for (int i = 0; i < n; i++) {
            result[i] = prefix[i] * suffix[i];
        }

        return result;
    }
};
