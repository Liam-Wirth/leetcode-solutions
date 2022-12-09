/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode() {}
 *     TreeNode(int val) { this.val = val; }
 *     TreeNode(int val, TreeNode left, TreeNode right) {
 *         this.val = val;
 *         this.left = left;
 *         this.right = right;
 *     }
 * }
 */
class Solution {
    public TreeNode invertTree(TreeNode root) {
       invertTreeRec(root);
       return root;
    }
private void invertTreeRec(TreeNode root){
    if(root == null) return;
    TreeNode temp = root.right;
    root.right = root.left;
    root.left = temp;
    invertTreeRec(root.left);
    invertTreeRec(root.right);

    }
}
