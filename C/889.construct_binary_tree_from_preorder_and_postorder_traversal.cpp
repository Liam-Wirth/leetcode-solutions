#include <vector>
#include <unordered_map>
using namespace std;

/**
 * Definition for a binary tree node.
 */
struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class Solution {
public:
    unordered_map<int, int> postIndexMap; // To store postorder indices for quick lookup
    int preIndex = 0; // Track position in preorder traversal

    TreeNode* constructTree(vector<int>& preorder, vector<int>& postorder, int postStart, int postEnd) {
        if (preIndex >= preorder.size() || postStart > postEnd) return nullptr;

        // Create the root from preorder[preIndex]
        TreeNode* root = new TreeNode(preorder[preIndex++]);

        // Base case: if there's only one node left, return it
        if (postStart == postEnd) return root;

        // The next element in preorder is the root of the left subtree
        int leftRootVal = preorder[preIndex];

        // Find the index of leftRootVal in postorder
        int leftSubtreeEnd = postIndexMap[leftRootVal];

        // Recursively build left and right subtrees
        root->left = constructTree(preorder, postorder, postStart, leftSubtreeEnd);
        root->right = constructTree(preorder, postorder, leftSubtreeEnd + 1, postEnd - 1);

        return root;
    }

    TreeNode* constructFromPrePost(vector<int>& preorder, vector<int>& postorder) {
        // Build a map for quick lookup of postorder indices
        for (int i = 0; i < postorder.size(); i++) {
            postIndexMap[postorder[i]] = i;
        }

        return constructTree(preorder, postorder, 0, postorder.size() - 1);
    }
};
