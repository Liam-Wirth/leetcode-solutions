
#include <utility>
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right)
      : val(x), left(left), right(right) {}
};

class Solution {
public:
  TreeNode *reverseOddLevels(TreeNode *root) {
    reverseOdd(root->left, root->right, 1);
    return root;
  }

private:
  void *reverseOdd(TreeNode *left, TreeNode *right, int calls) {
    if (left == nullptr || right == nullptr)
      return nullptr;
    if (calls % 2 == 1) {
      int temp = left->val;
      std::swap(left->val, right->val);
    }
    reverseOdd(left->left, right->right, calls + 1);
    reverseOdd(left->right, right->left, calls + 1);
    return nullptr;
  }
};
